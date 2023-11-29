#[macro_export]
macro_rules! kib {
    ($size:expr) => {
        $size * 1024
    };
    () => {};
}

/// auto deref macro.
/// to access `self` use `_self` in the expression.
/// 
/// # Example
/// 
/// ```
/// deref!(ID: Uuid |_self| => &Uuid::from_bytes_ref(&_self.0));
/// ```
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
            deref!(@CONSTRUCT $($rest)*);
        };

    (@CONSTRUCT $ident:tt: $target:tt $self:ident $expr:expr; $($rest:tt)*) => {
            impl std::ops::Deref for $ident {
                type Target = $target;

                fn deref(&self) -> &Self::Target {
                    let $self =self;
                    $expr
                }
            }
            deref!(@CONSTRUCT $($rest)*);
        };

    // handle single case
    ($ident:tt: $target:tt $(|$self:ident| => $expr:expr)?) => {
        deref!(@CONSTRUCT $ident: $target $($self)? $($expr)?;);
    };

    // handle multiple cases
    ($($ident:tt: $target:tt;)*) => {
        deref!(@CONSTRUCT $($ident: $target;)*);
    };
}
