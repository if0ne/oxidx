#[doc(hidden)]
#[macro_export]
macro_rules! create_type {
    ($name:ident wrap $raw_type:ty) => {
        create_type! { $name wrap $raw_type; decorator for }
    };
    ($name:ident wrap $raw_type:ty; decorator for $( $base:ty ),*) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name($raw_type);

        impl $crate::HasInterface for $name {
            type Raw = $raw_type;
            type RawRef<'a> = &'a $raw_type;

            fn new(raw: Self::Raw) -> Self {
                Self(raw)
            }

            fn as_raw(&self) -> &Self::Raw {
                &self.0
            }

            fn as_raw_ref(&self) -> Self::RawRef<'_> {
                &self.0
            }
        }

        $(
            impl TryInto<$name> for $base {
                type Error = $crate::error::DxError;

                fn try_into(self) -> Result<$name, Self::Error> {
                    let temp = self.0.cast::<_>().map_err(|_| $crate::error::DxError::CastError)?;

                    Ok(<$name>::new(temp))
                }
            }
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_trait {
    (impl $interface:ty => $( $t:ty ),+; $( $func:item )* ) => {
        impl_trait! { @impl_tuple impl $interface => $($t),+; ($($func),*) }
    };
    (@impl_tuple impl $interface:ty => $( $t:ty ),*; $tuple:tt ) => {
        $(impl_trait! { @impl_fn impl $interface => $t; $tuple } )+
    };
    (@impl_fn impl $interface:ty => $t:ty; ($($func:item),*)) => {
        impl $interface for $t {
            $(
                $func
            )*
        }
    }
}