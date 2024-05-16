#[macro_export]
macro_rules! create_types {
    ($(($name:ident, $raw_type:ty)),+ $(,)?) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub(crate) struct Inner<T: windows::core::Interface>(pub(crate) T);

        impl<T: windows::core::Interface> std::ops::Deref for Inner<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        $(
            #[allow(dead_code)]
            pub struct $name(Inner<$raw_type>);

            impl $name {
                #[allow(dead_code)]
                pub(crate) fn new(inner: $raw_type) -> Self {
                    Self(Inner(inner))
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! allow_casting {
    ($src:ty, $dst:ty) => {
        impl TryInto<$dst> for $src {
            type Error = DxError;

            fn try_into(self) -> Result<$dst, Self::Error> {
                let temp = self.0.cast::<_>().map_err(|_| DxError::CastError)?;

                Ok(<$dst>::new(temp))
            }
        }
    };
}
