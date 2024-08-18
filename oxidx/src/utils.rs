#[doc(hidden)]
#[macro_export]
macro_rules! create_type {
    ($(#[$attr:meta])* $name:ident wrap $raw_type:ty) => {
        create_type! { $(#[$attr])* $name wrap $raw_type; decorator for }
    };
    ($(#[$attr:meta])* $name:ident wrap $raw_type:ty; decorator for $( $base:ty ),*) => {
        $(#[$attr])*
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $name(pub(crate) $raw_type);

        impl $crate::HasInterface for $name {
            type Raw = $raw_type;
            type RawRef<'a> = &'a $raw_type;

            #[inline]
            fn new(raw: Self::Raw) -> Self {
                Self(raw)
            }

            #[inline]
            fn as_raw(&self) -> &Self::Raw {
                &self.0
            }

            #[inline]
            fn as_raw_ref(&self) -> Self::RawRef<'_> {
                &self.0
            }
        }

        $(
            /// Upcast
            impl From<$name> for $base {
                #[inline]
                fn from(value: $name) -> Self {
                    unsafe { <$base>::new(value.0.cast::<_>().unwrap_unchecked()) }
                }
            }

            /// Downcast
            impl TryFrom<$base> for $name {
                type Error = $crate::error::DxError;

                #[inline]
                fn try_from(value: $base) -> Result<$name, Self::Error> {
                    let temp = value.0.cast::<_>()
                        .map_err(|_|
                            $crate::error::DxError::Cast(
                                std::any::type_name::<$base>(),
                                std::any::type_name::<$name>()
                            ))?;

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

#[doc(hidden)]
#[macro_export]
macro_rules! conv_enum {
    ($h:ident to $l:ident) => {
        impl $h {
            #[inline]
            pub(crate) fn as_raw(&self) -> $l {
                $l(*self as i32)
            }
        }

        impl From<$l> for $h {
            #[inline]
            fn from(value: $l) -> Self {
                $h::from_repr(value.0).unwrap_or_else(|| unreachable!())
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! conv_flags {
    ($h:ident to $l:ident) => {
        impl $h {
            #[inline]
            pub(crate) fn as_raw(&self) -> $l {
                $l(self.bits())
            }
        }

        impl From<$l> for $h {
            #[inline]
            fn from(value: $l) -> Self {
                Self::from_bits(value.0).unwrap_or_else(|| unreachable!())
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_up_down_cast {
    ($child:ident inherit $base:ty) => {
        /// Upcast
        impl From<$child> for $base {
            #[inline]
            fn from(value: $child) -> Self {
                unsafe { <$base>::new(value.0.cast::<_>().unwrap_unchecked()) }
            }
        }

        /// Downcast
        impl TryFrom<$base> for $child {
            type Error = $crate::error::DxError;

            #[inline]
            fn try_from(value: $base) -> Result<$child, Self::Error> {
                let temp = value.0.cast::<_>().map_err(|_| {
                    $crate::error::DxError::Cast(
                        std::any::type_name::<$base>(),
                        std::any::type_name::<$child>(),
                    )
                })?;

                Ok(<$child>::new(temp))
            }
        }
    };
}
