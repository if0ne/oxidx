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
        pub struct $name(pub $raw_type);

        impl AsRef<$name> for $name {
            fn as_ref(&self) -> &Self {
                self
            }
        }

        $(
            /// Upcast
            impl From<$name> for $base {
                #[inline]
                fn from(value: $name) -> Self {
                    unsafe { Self(value.0.cast::<_>().unwrap_unchecked()) }
                }
            }

            impl AsRef<$base> for $name {
                fn as_ref(&self) -> &$base {
                    unsafe { &*(self as *const $name as *const $base) }
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

                    Ok(Self(temp))
                }
            }
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_interface {
    ($( $t:ty ),+; $( $func:item )* ) => {
        impl_interface! { @impl_tuple $($t),+; ($($func),*) }
    };
    (@impl_tuple $( $t:ty ),*; $tuple:tt ) => {
        $(impl_interface! { @impl_fn $t; $tuple } )+
    };
    (@impl_fn $t:ty; ($($func:item),*)) => {
        impl $t {
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
                unsafe { Self(value.0.cast::<_>().unwrap_unchecked()) }
            }
        }

        /// Downcast
        impl From<$base> for $child {
            #[inline]
            fn from(value: $base) -> Self {
                let temp = value.0.cast::<_>().expect(&format!(
                    "failed to cast from {} to {}",
                    std::any::type_name::<$base>(),
                    std::any::type_name::<$child>(),
                ));

                Self(temp)
            }
        }
    };
}
