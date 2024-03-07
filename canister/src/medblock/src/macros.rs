#[macro_export]
macro_rules! kib {
    ($size:expr) => {
        $size * 1024
    };
    () => {};
}

/// auto [deref](std::ops::Deref) macro.
/// meant to be used for newtypes.
/// to access `self` use `_self` in the expression.
///
/// the syntax is `<IDENT>: <TARGET> $(|<SELF>| => <EXPR>)`
/// ```
/// deref!(Foo: Bar |_self| => &Bar::from(&_self.0));
/// ```
/// or
/// `deref!(<IDENT>: <TARGET>;)` for direct implementation like below
/// ```
/// deref!(Foo: Bar;);
/// ```
/// to implement [derefmut](std::ops::DerefMut) add mut in front of the ident
/// ```
/// deref!(mut Foo: Bar;);
/// deref!(mut Foo: Bar |_self| => &mut Bar::from(&mut _self.0);
/// ```
#[macro_export]
macro_rules! deref {
    (@CONSTRUCT) => {};

    (@CONSTRUCT $ident:ty: $target:ty;) => {
            impl std::ops::Deref for $ident {
                type Target = $target;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
    };

    (@CONSTRUCT MUTABLE $ident:ty: $target:ty;) => {
            impl std::ops::DerefMut for $ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
    };

    (@CONSTRUCT $ident:tt: $target:tt $self:ident $expr:expr;) => {
            impl std::ops::Deref for $ident {
                type Target = $target;

                fn deref(&self) -> &Self::Target {
                    let $self =self;
                    $expr
                }
            }
    };

    (@CONSTRUCT MUTABLE $ident:tt: $target:tt $self:ident $expr:expr;) => {
            impl std::ops::DerefMut for $ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    let $self =self;
                    $expr
                }
            }
    };

    ($ident:tt: $target:tt $(| $self:ident | => $expr:expr)?) => {
        deref!(@CONSTRUCT $ident: $target $($self)? $($expr)?;);
    };

    (mut $ident:tt: $target:tt $(| $self:ident | => $expr:expr)?) => {
        deref!(@CONSTRUCT $ident: $target $($self)? $($expr)?;);
        deref!(@CONSTRUCT MUTABLE $ident: $target $($self)? $($expr)?;);
    };

    ($ident:tt: $target:ty) => {
        deref!(@CONSTRUCT $ident: $target;);
    };

    (mut $ident:tt: $target:ty) => {
        deref!(@CONSTRUCT $ident: $target;);
        deref!(@CONSTRUCT MUTABLE $ident: $target;);
    };
}

/// macro to measure stable memory allocation.
/// make sure to return the stable memory object if you're passing a code block like below
/// ```
/// measure_alloc!("records": {
///       let mut records = Records::default();
///
///       records.insert(
///           AsciiRecordsKey::new("test".to_string()).unwrap(),
///           EmrRecordsValue::new("test").unwrap(),
///       );
///
///       // return the stable memory object
///       records
/// });
///
/// ```
/// or if the type implement [Default] you can pass the type directly like this
/// ```
/// measure_alloc!(Foo);
/// ```
/// will panics for now to print the allocated size.
#[macro_export]
macro_rules! measure_alloc {
    ($ty:ty) => {
        paste::paste! {
            #[cfg(test)]
            mod [<__measure_alloc_ $ty:lower>] {
                use super::*;

                #[test]
                fn measure_alloc(){
                    ic_stable_memory::stable_memory_init();

                    $ty::default();

                    let allocated = ic_stable_memory::get_allocated_size();
                    panic!("total allocated for types  {} : {} bytes", stringify!($ty) ,allocated);

                }
            }
        }
    };

    ($id:literal: $block:block) => {
        paste::paste! {
            #[cfg(test)]
            mod [<__measure_alloc_ $id:lower>] {
                use super::*;

                #[test]
                fn measure_alloc(){
                    ic_stable_memory::stable_memory_init();

                    let _b = $block;

                    let allocated = ic_stable_memory::get_allocated_size();
                    println!("total allocated for id {} types: {} megabytes", stringify!($id), allocated / 1024 / 1024);
                    println!("total allocated for id {} types: {} kilobytes", stringify!($id), allocated / 1024);
                    println!("total allocated for id {} types: {} bytes", stringify!($id), allocated);
                    panic!("allocation test success");
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_max_size {
    (for $struct:ty: $($ty:ident),*) => {

        impl $struct {
            pub const fn max_size()-> usize {
                0 $(+ std::mem::size_of::<$ty>())*
            }
        }
    };

    (for $struct:ty: $lit:tt) => {
        impl $struct {
            pub const fn max_size() -> usize {
                $lit
            }
        }
    };
}

/// macro to implement [crate::mem::shared::MemBoundMarker] for a type.
/// to use the bounded macro, make sure to use impl max size first
#[macro_export]
macro_rules! impl_mem_bound {
    (for $struct:ty: unbounded) => {
        impl $crate::mem::shared::MemBoundMarker for $struct {
            const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
        }
    };

    (for $struct:ty: bounded; fixed_size: $lit:literal) => {
        impl $crate::mem::shared::MemBoundMarker for $struct {
    
            const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded { max_size: <$struct>::max_size() as u32, is_fixed_size: $lit };
        }
    };
}

#[macro_export]
macro_rules! zero_sized_state {
    ($($ident:ident),*) => {
        $(
            #[derive(Debug, Clone, Default)]
            pub struct $ident;

            impl $crate::emr::key::UsageMarker for $ident {}

        )*
    };
}

#[macro_export]
macro_rules! impl_range_bound {
    ($($ident:ident),*) => {
        
        $(
            impl std::ops::RangeBounds<$ident> for $ident {
                fn start_bound(&self) -> std::ops::Bound<&$ident> {
                    std::ops::Bound::Included(self)
                }
        
                fn end_bound(&self) -> std::ops::Bound<&$ident> {
                    std::ops::Bound::Excluded(self)
                }
            }
        )*
    };
}
