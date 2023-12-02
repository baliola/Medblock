use std::str::FromStr;

use candid::CandidType;
use ic_stable_memory::derive::{AsFixedSizeBytes, StableType};

use crate::deref;
use serde::Deserialize;
use uuid::Uuid;

/// timestamp in nanoseconds
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
    Copy,
    Deserialize,
)]
pub struct Timestamp(u64);

impl Timestamp {
    /// returns the current time in nanoseconds
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self(ic_cdk::api::time())
    }
}
/// emr metadata key must not exceed 100 ascii characters
#[derive(
    StableType, AsFixedSizeBytes, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug, CandidType,
)]
pub struct EmrRecordsKey([u8; EMR_RECORDS_MAX_LEN_BYTES]);
/// for some reason [CandidType] only supports fixed size arrays up to 32 bytes
const EMR_RECORDS_MAX_LEN_BYTES: usize = 32;
deref!(EmrRecordsKey: [u8; EMR_RECORDS_MAX_LEN_BYTES]);

/// wrapper for [uuid::Uuid] because candid is not implemented for [uuid::Uuid]
#[derive(
    StableType, AsFixedSizeBytes, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug, CandidType,
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

impl Into<Uuid> for Id {
    fn into(self) -> Uuid {
        Uuid::from_bytes(self.0)
    }
}

deref!(Id: Uuid |_self| => &Uuid::from_bytes_ref(&_self.0));

mod deserialize {
    use super::*;

    impl<'de> serde::Deserialize<'de> for Id {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let uuid = Uuid::parse_str(&s).map_err(serde::de::Error::custom)?;
            Ok(Self::from(uuid))
        }
    }

    impl<'de> serde::Deserialize<'de> for EmrRecordsKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            if !deserializer.is_human_readable() {
                return Err(serde::de::Error::custom("key must be a ascii string"));
            }

            let mut s = String::deserialize(deserializer)?;

            if !s.is_ascii() {
                return Err(serde::de::Error::custom("key must be ascii"));
            }

            s.make_ascii_lowercase();

            if s.len() > EMR_RECORDS_MAX_LEN_BYTES {
                return Err(serde::de::Error::custom(
                    "key must not exceed 100 ascii characters",
                ));
            }
            // TODO: unnecessary copy
            let mut key = [0u8; EMR_RECORDS_MAX_LEN_BYTES];
            key[..s.len()].copy_from_slice(s.as_bytes());

            Ok(Self(key))
        }
    }
}
