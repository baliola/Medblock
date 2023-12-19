use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(HeapClone)]
pub fn heap_clone_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let fields = match input.data {
        syn::Data::Struct(ref data) => &data.fields,
        _ => {
            return syn::Error
                ::new(input.ident.span(), "HeapClone only works on structs")
                .to_compile_error()
                .into();
        }
    };

    todo!()
}
