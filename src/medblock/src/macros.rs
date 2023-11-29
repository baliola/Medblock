
#[macro_export]
macro_rules! kib {
    ($size:expr) => {
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

            deref!(@CONSTRUCT $($rest)*);
        };

    // handle single case
    ($ident:ty: $target:ty) => {
        deref!(@CONSTRUCT $ident: $target;);
    };

    // handle multiple cases
    ($($ident:ty: $target:ty;)*) => {
        deref!(@CONSTRUCT $($ident: $target;)*);
    };

}
