/// cutting boiler plate for implementing bounded traits on types
#[macro_export]
macro_rules! bounded {
    (@CONSTRUCT ) => {};


    (@CONSTRUCT $ident:tt:Unbounded; $($rest:tt)*) => {
        impl crate::wrapper::Bounded for $ident {
            const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
        }

        bounded!(@CONSTRUCT $($rest)*);
    };


    (@CONSTRUCT $ident:ident: $ty:ty; $($rest:tt)*) => {
            impl crate::wrapper::Bounded for $ident {
                const BOUND: ic_stable_structures::storable::Bound = <$ty as ic_stable_structures::Storable>::BOUND;
            }

            bounded!(@CONSTRUCT $($rest)*);
    };

    (@CONSTRUCT $ident:ty:{
        max_size: $max:expr,
        is_fixed: $is_fixed:expr,
    }; $($rest:tt)*)=>{
        impl crate::wrapper::Bounded for $ident {
            const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded{
                max_size: $max,
                is_fixed_size: $is_fixed,

            };
        }

        bounded!(@CONSTRUCT $($rest)*);

    };

    ($($ident:tt: $any_expr:tt;)*) => {
        bounded!(@CONSTRUCT $($ident: $any_expr;)*);
    };

}
#[macro_export]
macro_rules! kib {
    ($size:lit) => {
        $size * 1024
    };
    () => {};
}

#[macro_export]
macro_rules! deref {

    (@CONSTRUCT ) => {};

    (@CONSTRUCT $ident:ty: $target:ty; $($rest:tt)*) => {
            impl std::ops::Deref for $ident {
                type Target = $target;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for $ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            auto_deref!(@CONSTRUCT $($rest)*);
        };

    // handle single case
    ($ident:ty: $target:ty) => {
        auto_deref!(@CONSTRUCT $ident: $target;);
    };

    // handle multiple cases
    ($($ident:ty: $target:ty;)*) => {
        auto_deref!(@CONSTRUCT $($ident: $target;)*);
    };

}
