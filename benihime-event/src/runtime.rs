use std::ops::Deref;

pub struct RuntimeLocal<T: 'static> {
    #[doc(hidden)]
    pub __data: T,
}

impl<T> Deref for RuntimeLocal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.__data
    }
}

#[macro_export]
macro_rules! runtime_local {
    ($($(#[$attr:meta])* $vis: vis static $name:ident: $ty: ty = $init: expr;)*) => {
        $($(#[$attr])* $vis static $name: $crate::runtime::RuntimeLocal<$ty> = $crate::runtime::RuntimeLocal {
            __data: $init
        };)*
    };
}
