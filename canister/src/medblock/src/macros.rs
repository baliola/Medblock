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

    (
        @CONSTRUCT $ident:ty: $target:ty;
    ) => {
            impl std::ops::Deref for $ident {
                type Target = $target;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
    };

    (
        @CONSTRUCT MUTABLE $ident:ty: $target:ty;
    ) => {
            impl std::ops::DerefMut for $ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
    };

    (
        @CONSTRUCT $ident:tt: $target:tt $self:ident $expr:expr;
    ) => {
            impl std::ops::Deref for $ident {
                type Target = $target;

                fn deref(&self) -> &Self::Target {
                    let $self =self;
                    $expr
                }
            }
    };

    (
        @CONSTRUCT MUTABLE $ident:tt: $target:tt $self:ident $expr:expr;
    ) => {
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
