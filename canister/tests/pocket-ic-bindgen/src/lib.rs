use std::{ fs, path::{ Path, PathBuf }, process::Command };

use file::Source;

#[cfg(test)]
mod test_data;

const MODULE_SPLIT_IDENTIFIER: &'static str = "pub mod pocket_ic_bindings";
pub struct Builder;

impl Builder {
    pub fn build(declaration_folder: PathBuf) {
        let raw = file::Fs::find_and_read(declaration_folder);
        let fragments = parser::Parser::parse(
            raw
                .iter()
                .map(|s| s.content.clone())
                .collect()
        );

        let sources = code_gen::Gen
            ::gen(fragments)
            .into_iter()
            .zip(raw.into_iter())
            .map(|(source, path)| Source { content: source.to_string(), path: path.path.clone() })
            .collect::<Vec<_>>();

        for source in sources {
            file::Fs::write_unchecked(source.path.clone(), source.content);

            Command::new("rustfmt")
                .arg(source.path)
                .arg("--edition")
                .arg("2021")
                .output()
                .expect("failed to format file");
        }
    }
}

mod file {
    use std::path::{ Path, PathBuf };

    pub struct Fs;

    #[derive(Debug, Clone)]
    pub struct Source {
        pub path: PathBuf,
        pub content: String,
    }

    impl Fs {
        fn traverse_and_find_declarations(path: PathBuf) -> Vec<PathBuf> {
            // read declaratio directory recursively and return all avaiable declarations (except mod.rs)
            std::fs
                ::read_dir(path)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| !path.ends_with("mod.rs"))
                .map(|path| path.to_path_buf())
                .collect()
        }

        fn read_raw(path: Vec<PathBuf>) -> Vec<String> {
            path.iter()
                .map(|path| std::fs::read_to_string(path).unwrap())
                .collect()
        }

        pub(super) fn find_and_read(path: PathBuf) -> Vec<Source> {
            let declarations = Fs::traverse_and_find_declarations(path);
            let raw = Fs::read_raw(declarations.clone());

            declarations
                .into_iter()
                .zip(raw.into_iter())
                .map(|(path, content)| Source { path, content })
                .collect()
        }

        pub(super) fn write_unchecked(path: PathBuf, content: String) {
            std::fs::write(path, content).unwrap();
        }
    }

    #[cfg(test)]
    mod tests {
        use std::path::Path;

        use super::*;

        #[test]
        fn test_traverse_and_find_declarations() {
            let dir = std::env::current_dir().unwrap();
            let path = dir.join("src/test_data");
            let declarations = Fs::traverse_and_find_declarations(path);

            assert_eq!(declarations.len(), 2);
        }

        #[test]
        fn test_read_raw() {
            let dir = std::env::current_dir().unwrap();
            let path = dir.join("src/test_data");
            let declarations = Fs::traverse_and_find_declarations(path);
            let raw = Fs::read_raw(declarations);

            assert_eq!(raw.len(), 2);
        }
    }
}

mod code_gen {
    use crate::{ parser, MODULE_SPLIT_IDENTIFIER };

    pub struct Gen;

    pub(super) struct Source {
        pub source: proc_macro2::TokenStream,
        pub fragment: parser::Fragment,
    }

    impl Source {
        pub fn new(source: proc_macro2::TokenStream, fragment: parser::Fragment) -> Self {
            Self { source, fragment }
        }

        fn to_token_stream(&self) -> proc_macro2::TokenStream {
            let source = self.source.clone();
            let fragment = self.fragment.to_token_stream();
            let boilerplate = Gen::static_boiler_plate();

            quote::quote! {
                #source
                
                pub mod pocket_ic_bindings {
                    use super::*;
                    use pocket_ic;
                    

                    #boilerplate
                    #fragment
                }
            }
        }
    }

    impl Gen {
        fn static_boiler_plate() -> proc_macro2::TokenStream {
            quote::quote! {
                fn call_pocket_ic<
                    R: candid::CandidType + serde::de::DeserializeOwned,
                    A: candid::CandidType + serde::de::DeserializeOwned
                >(
                    s: &pocket_ic::PocketIc,
                    f: impl FnOnce(
                        &pocket_ic::PocketIc,
                        ic_principal::Principal,
                        ic_principal::Principal,
                        &str,
                        Vec<u8>
                    ) -> std::result::Result<pocket_ic::WasmResult, pocket_ic::UserError>,
                    id: ic_principal::Principal,
                    sender: ic_principal::Principal,
                    method: &str,
                    payload: A
                ) -> std::result::Result<R, pocket_ic::UserError> {
                    use candid::Decode;
                    use candid::Encode;

                    let args = Encode!(&payload).unwrap();

                    let result = f(s, id, sender, method, args);

                    match result {
                        Ok(r) =>
                            match r {
                                pocket_ic::WasmResult::Reply(vec) => { Ok(Decode!(vec.as_slice(), R).unwrap()) }
                                pocket_ic::WasmResult::Reject(e) => panic!("Error: {:?}", e),
                            }
                        Err(e) => Err(e),
                    }
                }

                #[derive(Clone, Debug, Deserialize, CandidType, PartialEq, Eq, PartialOrd, Ord)]
                pub enum Call {
                    Query,
                    Update,
                }
            }
        }

        pub fn gen(raw: Vec<Source>) -> Vec<proc_macro2::TokenStream> {
            raw.into_iter()
                .map(|source| source.to_token_stream())
                .collect()
        }
    }
}

mod parser {
    use std::str::FromStr;

    use proc_macro2::TokenStream;
    use quote::ToTokens;

    use super::*;

    macro_rules! must_type {
        ($ident:expr, $variant:path $(, $lit:literal)?) => {
        match $ident {
            $variant(v) => v,
            _ => panic!($($lit)?)
        }
        };
    }

    #[derive(Debug, Clone)]
    struct Arg {
        name: Option<String>,
        ty: String,
        ampersand: bool,
        is_return_args: bool,
    }

    impl Arg {
        fn is_reference(&self) -> bool {
            self.ampersand
        }

        fn to_token_stream(&self) -> proc_macro2::TokenStream {
            if self.is_return_args {
                return self.to_type_only();
            }

            let ty = self.ty.as_str();
            let ty = syn::parse_str::<syn::Type>(ty).unwrap();

            let name = self.name
                .as_ref()
                .map(|name| name.as_str())
                .unwrap();

            let name = syn::parse_str::<syn::Ident>(name).unwrap();

            if self.ampersand {
                quote::quote! { #name: &#ty }
            } else {
                quote::quote! { #name: #ty }
            }
        }

        fn to_token_stream_with_numbered_name(
            &self,
            number: usize
        ) -> (proc_macro2::TokenStream, String) {
            if self.name.is_some() {
                return (self.to_token_stream(), self.name.clone().unwrap());
            }

            let ty = self.ty.as_str();
            let name = format!("arg{}", number);
            let name_ident = name.clone();
            let name = proc_macro2::token_stream::TokenStream::from_str(&name).unwrap();

            (quote::quote!(#name: #ty), name_ident)
        }

        fn to_type_only(&self) -> proc_macro2::TokenStream {
            let ty = syn::parse_str::<syn::Type>(&self.ty).unwrap();
            quote::quote!(#ty)
        }
    }

    #[derive(Debug, Clone)]
    struct Method {
        name: String,
        /// the actual candid args that must be passed to pocket ic
        args: Vec<Arg>,
        /// the return args that's returned from the canister call
        return_args: Vec<Arg>,
    }

    impl Method {
        fn to_token_stream(&self) -> proc_macro2::TokenStream {
            let name = self.name.as_str();
            let name = syn::parse_str::<syn::Ident>(name).unwrap();

            let args = self.args
                .iter()
                .enumerate()
                .map(|(index, arg)| arg.to_token_stream_with_numbered_name(index))
                .collect::<Vec<_>>();

            let (args, idents): (Vec<TokenStream>, Vec<String>) = args.iter().cloned().unzip();
            let idents = idents
                .iter()
                .map(|s| syn::parse_str::<syn::Ident>(s).unwrap())
                .collect::<Vec<_>>();

            let ids = idents.clone();
            let payload = match ids.is_empty() {
                true => quote::quote! { () },
                false => quote::quote! { (#(#ids),*,) },
            };

            let return_args = self.return_args
                .iter()
                .map(|arg| arg.to_token_stream())
                .collect::<Vec<_>>();

            let default_return_args = syn
                ::parse_str::<syn::TypeTuple>("()")
                .unwrap()
                .to_token_stream();

            let return_args = return_args.first().unwrap_or_else(|| &default_return_args);

            let method_name = name.to_string();

            quote::quote! {
                pub fn #name(
                    &self,  
                    server: &pocket_ic::PocketIc,
                    sender: ic_principal::Principal,
                    call_type: Call,
                    #(#args),*
                ) -> std::result::Result<#return_args, pocket_ic::UserError> {
            
                    let f = match call_type {
                        Call::Query => pocket_ic::PocketIc::query_call,
                        Call::Update => pocket_ic::PocketIc::update_call,
                    };

                    let payload = #payload;                
                    call_pocket_ic(server, f, self.0.clone(), sender, #method_name, payload)

                }
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(super) struct Fragment {
        name: String,
        methods: Vec<Method>,
    }

    impl Fragment {
        pub fn to_token_stream(&self) -> proc_macro2::TokenStream {
            let name = syn::parse_str::<syn::Ident>(&self.name).unwrap();
            let methods = self.methods
                .iter()
                .map(|method| method.to_token_stream())
                .collect::<Vec<_>>();

            quote::quote! {
                pub struct #name(pub ic_principal::Principal);
                impl #name {
                    #(#methods)*
                }
            }
        }
    }

    pub struct Parser;

    impl Parser {
        pub fn parse(raw: Vec<String>) -> Vec<code_gen::Source> {
            let raw = raw.into_iter().map(Self::slice_source).collect::<Vec<_>>();
            let fragments = raw.clone().into_iter().map(Self::parse_one).collect::<Vec<_>>();

            raw.into_iter()
                .zip(fragments.into_iter())
                .map(|(source, fragment)|
                    code_gen::Source::new(
                        syn::parse_str::<syn::File>(&source).unwrap().to_token_stream(),
                        fragment
                    )
                )
                .collect()
        }
        /// parse only till the header comment if exists, and ignore the rest,
        fn slice_source(raw: String) -> String {
            raw.split_terminator(MODULE_SPLIT_IDENTIFIER)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()[0]
                .clone()
        }

        fn parse_one(raw: String) -> Fragment {
            let items = syn::parse_file(&raw).unwrap().items;
            let registry_name = items
                .iter()
                .filter(|item| matches!(item, syn::Item::Impl(_)))
                .collect::<Vec<_>>();

            let mut fragments = Fragment {
                name: Default::default(),
                methods: vec![],
            };

            // parse the name
            let registry_name = registry_name.first().unwrap();
            let impls = must_type!(registry_name, syn::Item::Impl, "unexpected item found");

            // quick hack as its guaranteed (atleast for now) that the impl only refer to the type directly, we can skip the pattern matching
            let name = impls.self_ty.to_token_stream().to_string();
            fragments.name = name;

            // parse methods
            let methods = impls.items
                .iter()
                .filter(|item| matches!(item, syn::ImplItem::Fn(_)))
                .map(|item| {
                    must_type!(item, syn::ImplItem::Fn, "unexpected item found, expected Fn")
                })
                .map(|method| Method {
                    name: method.sig.ident.to_string(),
                    args: method.sig.inputs
                        .iter()
                        .filter(|s| !matches!(s, syn::FnArg::Receiver(_)))
                        .map(Self::parse_input)
                        .collect(),
                    return_args: Self::parse_output(method.sig.output.clone()),
                });

            fragments.methods = methods.collect();

            fragments
        }

        // aimed to handle return type such as Result<(T,)>
        fn parse_output(output: syn::ReturnType) -> Vec<Arg> {
            let output = match output {
                syn::ReturnType::Type(_, ty) => { must_type!(*ty, syn::Type::Path) }
                syn::ReturnType::Default => panic!("unexpected return type got : ()"),
            };

            let output = output.path.segments.first().unwrap().arguments.clone();

            let output = must_type!(
                output,
                syn::PathArguments::AngleBracketed,
                "unexpected path arguments found"
            );

            let output = output.args.first().unwrap();

            let output = must_type!(
                output,
                syn::GenericArgument::Type,
                "unexpected generic argument found"
            );

            let output = must_type!(output, syn::Type::Tuple, "unexpected type found");

            let Some(output) = output.elems.first() else {
                return vec![];
            };

            let output = must_type!(output, syn::Type::Path, "unexpected type found")
                .path.get_ident()
                .unwrap()
                .to_string();

            vec![Arg {
                name: None,
                ty: output,
                ampersand: false,
                is_return_args: true,
            }]
        }

        fn parse_input(arg: &syn::FnArg) -> Arg {
            match arg.clone() {
                syn::FnArg::Receiver(r) =>
                    Arg {
                        name: None,
                        ty: r.self_token.to_token_stream().to_string(),
                        ampersand: r.reference.is_some(),
                        is_return_args: false,
                    },
                syn::FnArg::Typed(r) =>
                    Arg {
                        name: Some(
                            must_type!(
                                *r.pat,
                                syn::Pat::Ident,
                                "unexpected pattern found"
                            ).ident.to_string()
                        ),
                        ty: r.ty.to_token_stream().to_string(),
                        // auto set to false for now
                        ampersand: false,
                        is_return_args: false,
                    },
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_parse_header() {
            let raw =
                "impl Registry {
                fn a() -> Result<(String, SomeType)> {}
                fn b() -> Result<(String, SomeType)> {}
            }";

            let raw_with_header = format!("{}{}", raw, MODULE_SPLIT_IDENTIFIER);
            let fragments: Vec<code_gen::Source> = Parser::parse(vec![raw_with_header.to_string()]);
            assert_eq!(fragments.len(), 1);
            assert!(!raw.contains(MODULE_SPLIT_IDENTIFIER));
        }

        #[test]
        fn test_parse_output() {
            let raw = "-> Result<(String, SomeType)>";
            let output = syn::parse_str::<syn::ReturnType>(raw).unwrap();
            let args = Parser::parse_output(output);

            assert_eq!(args.len(), 1);
            assert_eq!(args[0].name, None);
            assert_eq!(args[0].ty, "String");
            assert_eq!(args[0].is_reference(), false);
        }

        #[test]
        fn test_parse_input() {
            let raw = "fn a(&self, b: String) -> Result<(String, SomeType)> {}";
            let item = syn::parse_str::<syn::ImplItemFn>(raw).unwrap();
            let mut args = vec![];

            for arg in item.sig.inputs.iter() {
                args.push(Parser::parse_input(arg));
            }

            assert_eq!(args.len(), 2);
            assert_eq!(args[0].name, None);
            assert_eq!(args[0].ty, "self");
            assert_eq!(args[0].is_reference(), true);

            assert_eq!(args[1].name, Some("b".to_string()));
            assert_eq!(args[1].ty, "String");
            assert_eq!(args[1].is_reference(), false);
        }

        #[test]
        fn test_parse() {
            let raw =
                "impl Registry {
                fn a() -> Result<(String, SomeType)> {}
                fn b() -> Result<(String, SomeType)> {}
            }";

            let fragments = Parser::parse_one(raw.to_string());

            assert_eq!(fragments.methods.len(), 2);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let dir = std::env::current_dir().unwrap();
        let path = dir.join("src/test_data");
        Builder::build(path);
    }
}
