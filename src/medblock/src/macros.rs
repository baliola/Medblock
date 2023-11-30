#[macro_export]
macro_rules! kib {
    ($size:expr) => {
        $size * 1024
    };
    () => {};
}

/// auto deref macro.
/// meant to be used for newtypes.
/// to access `self` use `_self` in the expression.
///
/// the syntax is `<IDENT>: <TARGET> $(|<SELF>| => <EXPR>)` for single line implementation like below
/// ```
/// deref!(Foo: Bar |_self| => &Bar::from(&_self.0));
///
///
/// ```
/// or
/// `deref!(<IDENT>: <TARGET>;)` for direct implementation like below
/// ```
/// deref!(Foo: Bar;);
/// ```
///
/// can also be combined like below
/// ```
/// deref!{
///   Foo: Bar;
///   Baz: Qux |_self| => &Qux::from(&_self.0);
///    ...}
/// ```
#[macro_export]
macro_rules! deref {
    (@CONSTRUCT) => {};

    (
        @CONSTRUCT $ident:ty: $target:ty;
        $($rest:tt)*
    ) => {
            impl std::ops::Deref for $ident {
                type Target = $target;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            deref!(@CONSTRUCT $($rest)*);
    };

    (
        @CONSTRUCT $ident:tt: $target:tt $self:ident $expr:expr;
        $($rest:tt)*
    ) => {
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
    ($ident:tt: $target:tt $(| $self:ident | => $expr:expr)?) => {
        deref!(@CONSTRUCT $ident: $target $($self)? $($expr)?;);
    };

    // handle multiple cases
    ($($ident:tt: $target:tt;)*) => {
        deref!(@CONSTRUCT $($ident: $target;)*);
    };
    
    ($ident:tt: $target:ty) => {
        deref!(@CONSTRUCT $ident: $target;);
    };
}
