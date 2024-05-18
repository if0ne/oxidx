#[macro_export]
macro_rules! create_type {
    ($interface:ty => $name:ident wrap $raw_type:ty; decorator for $( $base:ty ),*) => {
        #[allow(dead_code)]
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name($raw_type);

        #[allow(dead_code)]
        impl $name {
            pub(crate) fn new(inner: $raw_type) -> Self {
                Self(inner)
            }
        }

        impl $crate::HasInterface for $name {
            type Raw = $raw_type;
            type RawRef<'a> = &'a $raw_type;

            fn as_raw_ref(&self) -> Self::RawRef<'_> {
                &self.0
            }
        }

        impl $interface for $name {}

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

#[macro_export]
macro_rules! create_type_with_param {
    ($interface:ty => $name:ident wrap $raw_type:ty : input as $param:ty; decorator for $( $base:ty ),*) => {

        create_type! { $interface => $name wrap $raw_type; decorator for $( $base ),* }

        impl $crate::IsParam<$param> for $name {
            fn as_raw_ref(&self) -> Self::RawRef<'_> {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! implement_fns {
    ($( $t:ty ),+; $( $func:item )+ ) => {
        implement_fns! { @impl_tuple $($t),+; ($($func),+) }
    };
    (@impl_tuple $( $t:ty ),+; $tuple:tt ) => {
        $(implement_fns! { @impl_fn $t; $tuple } )+
    };
    (@impl_fn $t:ty; ($($func:item),+)) => {
        impl $t {
            $(
                $func
            )+
        }
    }
}
