#[macro_export]
macro_rules! new_type {
    ($struct_name: ident, String) => {
        #[derive(bevy::prelude::Component, bevy::prelude::Reflect, Ord, PartialOrd, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize, Clone, Hash, bevy::prelude::Deref)]
        pub struct $struct_name(pub String);
        
        impl From<&str> for $struct_name {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }
        
        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
    ($struct_name: ident, $ty: ident) => {
        #[derive(bevy::prelude::Component, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize, Copy, Clone, Hash, bevy::prelude::Deref)]
        pub struct $struct_name(pub $ty);
   
        impl From<$ty> for $struct_name {
            fn from(value: $ty) -> Self {
                Self(value)
            }
        }
    };
}
