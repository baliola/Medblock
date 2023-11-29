use ic_stable_memory::{ collections::SHashMap, derive::{ AsFixedSizeBytes, StableType }, SBox };
use uuid::Uuid;

use crate::deref;

#[derive(
    CandidType,
    StableType,
    AsFixedSizeBytes,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug
)]
pub struct Timestamp(u64);

/// emr metadata key must not exceed 100 ascii characters
#[derive(
    Deserialize,
    CandidType,
    StableType,
    AsFixedSizeBytes,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug
)]
pub struct EmrRecordsKey([u8; 100]);
deref!(EmrRecordsKey: [u8; 100]);

/// wrapper for [uuid::Uuid] because candid is not implemented for [uuid::Uuid]
#[derive(
    CandidType,
    StableType,
    AsFixedSizeBytes,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug,
    Deserialize
)]
pub struct Id([u8; 16]);

impl Id {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Id {
    fn default() -> Self {
        uuid::Uuid::new_v4().into()
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value.into_bytes())
    }
}

deref!(Id: Uuid |_self| => &Uuid::from_bytes_ref(&_self.0));
