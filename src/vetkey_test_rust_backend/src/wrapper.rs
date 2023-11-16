use core::mem::size_of;

use ic_stable_structures::{storable::Bound, Storable};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// trait for types that have a bound, i.e. a maximum size
/// and whether they are fixed size or not
///
/// this must be implemented for inner types of [Serializeable] as the storable trait requires it
/// types to have a bounded information on them, but we can't flexibly implement it for all types
/// that [Serializeable] wraps. so the bounded information must be implemented manually.
pub trait Bounded {
    const BOUND: Bound;
}

// blanket impl for native types
impl<Data: Storable + Serialize + DeserializeOwned> Bounded for Data {
    const BOUND: Bound = <Data as Storable>::BOUND;
}

/// wrapper types for types that exist in the stable memory of the canister
///
/// NOTE: data must implements [Bounded] trait (execpt for native types that's already supported)
#[derive(Default, Serialize, PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
pub struct Stable<Data>(pub Data)
where
    Data: Serialize + DeserializeOwned + Bounded;

impl<Data> Stable<Data>
where
    Data: Serialize + DeserializeOwned + Bounded
{
    /// unwrap the inner data (consumes the wrapper)
    pub fn into_inner(self) -> Data {
        self.0
    }
}

impl<Data> From<Data> for Stable<Data>
where
    Data: Serialize + DeserializeOwned + Bounded,
{
    fn from(value: Data) -> Self {
        Self(value)
    }
}

impl<Data> std::ops::Deref for Stable<Data>
where
    Data: Serialize + DeserializeOwned + Bounded,
{
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Data> Storable for Stable<Data>
where
    Data: Serialize + DeserializeOwned + Bounded,
{
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut buffer = vec![];

        ciborium::ser::into_writer(&self.0, &mut buffer)
            .expect("type bounded serialization should not fail");

        std::borrow::Cow::Owned(buffer)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let data = ciborium::de::from_reader(bytes.as_ref())
            .expect("type bounded deserialization should not fail");

        Self(data)
    }

    const BOUND: Bound = <Data as Bounded>::BOUND;
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
    pub struct DummyEmployee {
        name: String,
        age: u32,
    }

    impl Bounded for DummyEmployee {
        const BOUND: Bound = Bound::Bounded {
            max_size: size_of::<Self>() as u32,
            is_fixed_size: true,
        };
    }

    impl DummyEmployee {
        pub fn new(name: String, age: u32) -> Self {
            Self { name, age }
        }
    }

    impl Default for DummyEmployee {
        fn default() -> Self {
            Self {
                name: "default".to_string(),
                age: 0,
            }
        }
    }

    #[test]
    fn test_serializeable_struct() {
        let employee = DummyEmployee::new("JohnAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string(), 30);
        let serialized = Stable(employee.clone());
        let serialized = serialized.to_bytes();
        let deserialized = Stable::<DummyEmployee>::from_bytes(serialized.clone());

        assert!(employee.eq(&deserialized))
    }

    #[test]
    fn test_serializeable_string() {
        let string = "JohnAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string();
        let serialized = Stable(string.clone());
        let serialized = serialized.to_bytes();
        let deserialized = Stable::<String>::from_bytes(serialized.clone());

        assert!(string.eq(&*deserialized))
    }

    #[test]
    fn test_serializeable_u32() {
        let u32 = 123456789;
        let serialized = Stable(u32);
        let serialized = serialized.to_bytes();
        let deserialized = Stable::<u32>::from_bytes(serialized.clone());

        assert!(u32.eq(&*deserialized))
    }
}
