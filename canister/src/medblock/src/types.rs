use std::str::FromStr;

use candid::CandidType;
use ic_stable_memory::{ derive::{ AsFixedSizeBytes, StableType }, primitive::s_ref::SRef };

use crate::{ deref };
use serde::{ Deserialize, Serialize };
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
    Serialize
)]
pub struct Timestamp(pub(crate) u64);

impl Timestamp {
    /// returns the current time in nanoseconds
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let time = chrono::Utc::now().timestamp_nanos_opt().unwrap();
            let time = u64::try_from(time).unwrap();
            Self(time)
        }

        #[cfg(target_arch = "wasm32")]
        {
            let time = ic_cdk::api::time();
            Self(time)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EmrKeyError {
    #[error("key must be a ascii string")]
    ContainsInvalidChars,

    #[error("key exceeded max emr records max length")]
    TooLong,
}

/// arbitry ascii encoded string with max length of 32 bytes
#[derive(
    StableType,
    AsFixedSizeBytes,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug,
    CandidType
)]
pub struct AsciiRecordsKey {
    key: [u8; EMR_RECORDS_MAX_LEN_BYTES],
    /// length of the key in bytes, used to exactly slice the correct bytes from the array and discard invalid bytes if exist
    len: u8,
}
/// for some reason [CandidType] only supports fixed size arrays up to 32 bytes
const EMR_RECORDS_MAX_LEN_BYTES: usize = 32;
deref!(AsciiRecordsKey: [u8; EMR_RECORDS_MAX_LEN_BYTES] |_self| => &_self.key);

impl AsciiRecordsKey {
    pub fn new(s: impl AsRef<str>) -> Result<Self, EmrKeyError> {
        Self::from_str(s.as_ref())
    }
}

impl FromStr for AsciiRecordsKey {
    type Err = EmrKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(EmrKeyError::ContainsInvalidChars);
        }

        let len = s.len();

        if len > EMR_RECORDS_MAX_LEN_BYTES {
            return Err(EmrKeyError::TooLong);
        }

        // TODO: duplicate code as serialization implementation
        let mut key = [0u8; EMR_RECORDS_MAX_LEN_BYTES];
        key[..s.len()].copy_from_slice(s.as_bytes());

        Ok(Self {
            key,
            len: len as u8,
        })
    }
}

impl ToString for AsciiRecordsKey {
    fn to_string(&self) -> String {
        self.to_ascii_str().to_string()
    }
}

impl AsciiRecordsKey {
    pub fn to_ascii_str(&self) -> &str {
        // discard invalid bytes
        let buffer_ref = &self.key[..self.len as usize];
        std::str::from_utf8(buffer_ref).expect("key must be ascii")
    }
}

/// wrapper for [uuid::Uuid] because candid is not implemented for [uuid::Uuid]
#[derive(StableType, AsFixedSizeBytes, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct Id([u8; 16]);

impl CandidType for Id {
    fn _ty() -> candid::types::Type {
        candid::types::Type::Text
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
        where S: candid::types::Serializer
    {
        // TODO : to_string() invloves copy
        serializer.serialize_text(self.to_string().as_str())
    }
}

impl Id {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Id {
    fn default() -> Self {
        // TODO : check that this works, as we dont know if it would actually fetch system time in canister execution environment
        uuid::Uuid::now_v7().into()
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value.into_bytes())
    }
}

impl From<Id> for Uuid {
    fn from(val: Id) -> Self {
        Uuid::from_bytes(val.0)
    }
}

deref!(Id: Uuid |_self| => Uuid::from_bytes_ref(&_self.0));

mod deserialize {
    use super::*;

    impl<'de> Serialize for AsciiRecordsKey {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
            serializer.serialize_str(self.to_ascii_str())
        }
    }

    impl<'de> serde::Deserialize<'de> for AsciiRecordsKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>
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
                return Err(serde::de::Error::custom("key must not exceed 32 ascii characters"));
            }
            // TODO: unnecessary copy
            let mut key = [0u8; EMR_RECORDS_MAX_LEN_BYTES];
            key[..s.len()].copy_from_slice(s.as_bytes());

            Ok(Self {
                key,
                len: s.len() as u8,
            })
        }
    }

    impl<'de> serde::Deserialize<'de> for Id {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>
        {
            Uuid::deserialize(deserializer).map(|uuid| uuid.into())
        }
    }

    impl<'de> serde::Serialize for Id {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
            Uuid::from_bytes_ref(&self.0).serialize(serializer)
        }
    }
}
