#[macro_export]
macro_rules! create_type {
    ($name:ident, $raw_type:ty) => {
       
        #[allow(dead_code)]
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name($raw_type);

        impl $name {
            #[allow(dead_code)]
            pub(crate) fn new(inner: $raw_type) -> Self {
                Self(inner)
            }
        }
    };
    ($name:ident, $raw_type:ty, $base:ty) => {
       
        #[allow(dead_code)]
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name($raw_type, $base);

        impl $name {
            #[allow(dead_code)]
            pub(crate) fn new(inner: $raw_type, base: $base) -> Self {
                Self(inner, base)
            }
        }

        impl TryInto<$name> for $base {
            type Error = DxError;

            fn try_into(self) -> Result<$name, Self::Error> {
                let temp = self.0.cast::<_>().map_err(|_| DxError::CastError)?;

                Ok(<$name>::new(temp, self))
            }
        }

        impl std::ops::Deref for $name {
            type Target = $base;

            fn deref(&self) -> &Self::Target {
                &self.1
            }
        }
    };
}

macro_rules! create_cast_sequence {
    () => {
        
    };
}