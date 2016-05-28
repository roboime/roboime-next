use ::Result;

pub trait InterfaceHandle {
    // TODO: rename to wait
    fn join(self) -> Result<()>;
    fn quit(&mut self) -> Result<()> { Ok(()) }
}

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        impl<$($name:InterfaceHandle),*> InterfaceHandle for ($($name,)*) {
            #[allow(non_snake_case, unused_assignments)]
            fn join(self) -> Result<()> {
                let ($($name,)*) = self;
                $(
                    try!($name.join());
                )*
                Ok(())
            }
        }
        peel! { $($name,)* }
    )
}

tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
