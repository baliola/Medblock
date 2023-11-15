use ic_stable_structures::{storable::Bound, Storable};
use serde::{de::DeserializeOwned, Serialize};

pub type MaxSize = u32;
pub type IsFixedSize = bool;

#[derive(Default)]
pub struct Serializeable<const size: MaxSize, const fix_sized_data: IsFixedSize, Data>(pub Data)
where
    Data: Serialize + DeserializeOwned;

impl<const size: MaxSize, const fix_sized_data: IsFixedSize, Data> std::ops::Deref
    for Serializeable<size, fix_sized_data, Data>
where
    Data: Serialize + DeserializeOwned,
{
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const size: MaxSize, const fix_sized_data: IsFixedSize, Data> Storable
    for Serializeable<size, fix_sized_data, Data>
where
    Data: Serialize + DeserializeOwned,
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

    const BOUND: Bound = Bound::Bounded {
        max_size: size,
        is_fixed_size: fix_sized_data,
    };
}
