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
/// ```no-test
/// deref!(Foo: Bar |_self| => &Bar::from(&_self.0));
/// ```
/// or
/// `deref!(<IDENT>: <TARGET>;)` for direct implementation like below
/// ```no-test
/// deref!(Foo: Bar;);
/// ```
/// to implement [derefmut](std::ops::DerefMut) add mut in front of the ident
/// ```no-test
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
        impl $crate::stable::MemBoundMarker for $struct {
            const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
        }
    };

    (for $struct:ty: bounded; fixed_size: $lit:literal) => {
        impl $crate::stable::MemBoundMarker for $struct {
    
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


#[macro_export]
macro_rules! native_bound {
    ($($ident:ident),*) => {
            $(
                paste::paste!{

                #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, parity_scale_codec::Encode, parity_scale_codec::Decode, Default)]
                pub struct [<Native $ident>](pub $ident);

               $crate::impl_max_size!(for [<Native $ident>]: $ident);
               $crate::impl_mem_bound!(for [<Native $ident>]: bounded; fixed_size: true);

                impl std::ops::RangeBounds<[<Native $ident>]> for [<Native $ident>] {
                    fn start_bound(&self) -> std::ops::Bound<&[<Native $ident>]> {
                        std::ops::Bound::Included(self)
                    }

                    fn end_bound(&self) -> std::ops::Bound<&[<Native $ident>]> {
                        std::ops::Bound::Excluded(self)
                    }
                }

                impl From<$ident> for [<Native $ident>] {
                    fn from(value: $ident) -> Self {
                        [<Native $ident>](value)
                    }
                }

                impl Into<$ident> for [<Native $ident>] {
                    fn into(self) -> $ident {
                        self.0
                    }
                }
                }

            )*
    };
}