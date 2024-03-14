use std::str::FromStr;

use candid::{ CandidType, Principal };
use ic_stable_structures::storable::Bound;
use parity_scale_codec::{ Decode, Encode };

use crate::{ deref, impl_max_size, impl_mem_bound, impl_range_bound, stable::MemBoundMarker };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

/// timestamp in nanoseconds
#[derive(
    CandidType,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug,
    Copy,
    Deserialize,
    Serialize,
    Encode,
    Decode
)]
pub struct Timestamp(pub(crate) u64);
impl_max_size!(for Timestamp: u64);
impl_mem_bound!(for Timestamp: bounded; fixed_size: true);

impl Timestamp {
    /// returns the current time in nanoseconds
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inner(&self) -> u64 {
        self.0
    }

    pub fn as_duration(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.0)
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

#[derive(thiserror::Error, Debug, CandidType)]
pub enum AsciiKeyError {
    #[error("key must be a ascii string")]
    ContainsInvalidChars,

    #[error("key exceeded max emr records max length")]
    TooLong,
}

/// arbitry ascii encoded string with max length of `N` bytes
#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Encode, Decode)]
pub struct AsciiRecordsKey<const N: usize = DEFAULT_RECORDS_LEN> {
    key: [u8; N],
    /// length of the key in bytes, used to exactly slice the correct bytes from the array and discard invalid bytes if exist
    // should probably make the check before initializing this struct so that it may be completely removed
    len: u8,
}

impl<const N: usize> MemBoundMarker for AsciiRecordsKey<N> {
    const BOUND: Bound = Bound::Bounded { max_size: Self::max_size() as u32, is_fixed_size: true };
}

impl<const N: usize> AsciiRecordsKey<N> {
    pub const fn max_size() -> usize {
        N + 1
    }
}

impl<const N: usize> Default for AsciiRecordsKey<N> {
    fn default() -> Self {
        Self { key: [0_u8; N], len: Default::default() }
    }
}

const DEFAULT_RECORDS_LEN: usize = 32;
deref!(AsciiRecordsKey: [u8; DEFAULT_RECORDS_LEN] |_self| => &_self.key);

impl<const N: usize> AsciiRecordsKey<N> {
    pub fn new(s: impl AsRef<str>) -> Result<Self, AsciiKeyError> {
        Self::from_str(s.as_ref())
    }
}

impl<const N: usize> FromStr for AsciiRecordsKey<N> {
    type Err = AsciiKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(AsciiKeyError::ContainsInvalidChars);
        }

        let len = s.len();

        if len > N {
            return Err(AsciiKeyError::TooLong);
        }

        // TODO: duplicate code as serialization implementation
        let mut key = [0u8; N];
        key[..s.len()].copy_from_slice(s.as_bytes());

        Ok(Self {
            key,
            len: len as u8,
        })
    }
}
impl<const N: usize> std::fmt::Display for AsciiRecordsKey<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_ascii_str().fmt(f)
    }
}

impl<const N: usize> AsciiRecordsKey<N> {
    pub fn to_ascii_str(&self) -> &str {
        // discard invalid bytes
        let buffer_ref = &self.key[..self.len as usize];
        std::str::from_utf8(buffer_ref).expect("key must be ascii")
    }
}

/// wrapper for [uuid::Uuid] because candid is not implemented for [uuid::Uuid]
#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Encode, Decode)]
pub struct Id([u8; 16]);
impl_max_size!(for Id: 16);
impl_mem_bound!(for Id: bounded; fixed_size: true);
impl_range_bound!(Id);

#[cfg(test)]
#[macro_export]
macro_rules! id {
    ($lit:literal) => {
        {

        let id = <uuid::Uuid as std::str::FromStr>::from_str($lit).unwrap();
        $crate::common::Id::from(id)
        }
    };
}

impl Default for Id {
    fn default() -> Self {
        uuid::Uuid::default().into()
    }
}

impl CandidType for Id {
    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
        where S: candid::types::Serializer
    {
        // TODO : to_string() invloves copy
        serializer.serialize_text(self.to_string().as_str())
    }

    fn _ty() -> candid::types::Type {
        candid::types::TypeInner::Text.into()
    }
}

/// max random bytes array len used to generate v7 uuid
pub(crate) const UUID_MAX_SOURCE_LEN: usize = 10;

impl Id {
    pub fn new(random_bytes: &[u8; UUID_MAX_SOURCE_LEN]) -> Self {
        let timestamp = Timestamp::new().as_duration();

        // safe to unwrap because timestamp is alyways less than u64::MAX
        let timestamp = u64::try_from(timestamp.as_millis()).unwrap();

        let uuid = uuid::Builder::from_unix_timestamp_millis(timestamp, random_bytes);
        let uuid = uuid.as_uuid().to_owned();

        uuid.into()
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value.into_bytes())
    }
}

impl From<&Uuid> for Id {
    fn from(value: &Uuid) -> Self {
        Self(*value.as_bytes())
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

    impl<const N: usize> CandidType for AsciiRecordsKey<N> {
        fn _ty() -> candid::types::Type {
            candid::types::TypeInner::Text.into()
        }

        fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
            where S: candid::types::Serializer
        {
            serializer.serialize_text(self.to_ascii_str())
        }
    }

    impl<'de, const N: usize> serde::Deserialize<'de> for AsciiRecordsKey<N> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>
        {
            let mut s = String::deserialize(deserializer)?;

            if !s.is_ascii() {
                return Err(serde::de::Error::custom("key must be ascii"));
            }

            s.make_ascii_lowercase();

            if s.len() > DEFAULT_RECORDS_LEN {
                return Err(serde::de::Error::custom("key must not exceed 32 ascii characters"));
            }
            // TODO: unnecessary copy
            let mut key = [0u8; N];
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
            let str = String::deserialize(deserializer)?;
            Uuid::parse_str(&str)
                .map_err(serde::de::Error::custom)
                .map(|uuid| uuid.into())
        }
    }

    impl<'de> serde::Serialize for Id {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
            Uuid::from_bytes_ref(&self.0).serialize(serializer)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{ Duration, SystemTime };
    use std::str::FromStr;

    #[test]
    fn test_timestamp_new() {
        let timestamp = Timestamp::new();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        std::thread::sleep(Duration::from_millis(500));

        let timestamp2 = Timestamp::new();
        let now2 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        assert!(timestamp.inner() <= now);
    }

    #[test]
    fn test_timestamp_inner() {
        let timestamp = Timestamp(1234567890);
        assert_eq!(timestamp.inner(), 1234567890);
    }

    #[test]
    fn test_timestamp_as_duration() {
        let timestamp = Timestamp(1234567890);
        assert_eq!(timestamp.as_duration(), Duration::from_nanos(1234567890));
    }

    #[test]
    fn test_ascii_records_key_new() {
        let key = AsciiRecordsKey::<32>::new("test").unwrap();
        assert_eq!(key.to_ascii_str(), "test");
    }

    #[test]
    fn test_ascii_records_key_from_str() {
        let key = AsciiRecordsKey::<32>::from_str("test").unwrap();
        assert_eq!(key.to_ascii_str(), "test");
    }

    #[test]
    fn test_ascii_records_key_to_ascii_str() {
        let key = AsciiRecordsKey::<32>::new("test").unwrap();
        assert_eq!(key.to_ascii_str(), "test");
    }

    #[test]
    fn test_id_new() {
        let uuid = Uuid::from_str("97780ca3-a626-4fc5-b150-7fa8bc665df6").unwrap();
        let id = id!("97780ca3-a626-4fc5-b150-7fa8bc665df6");
        assert_eq!(id.0.len(), 16);
    }

    #[test]
    fn test_id_from_uuid() {
        let uuid = Uuid::from_str("97780ca3-a626-4fc5-b150-7fa8bc665df6").unwrap();
        let id = id!("97780ca3-a626-4fc5-b150-7fa8bc665df6");
        assert_eq!(id.0, *uuid.as_bytes());
    }

    #[test]
    fn test_id_from_uuid_ref() {
        let uuid = Uuid::from_str("97780ca3-a626-4fc5-b150-7fa8bc665df6").unwrap();
        let id = id!("97780ca3-a626-4fc5-b150-7fa8bc665df6");
        assert_eq!(id.0, *uuid.as_bytes());
    }

    #[test]
    fn test_id_into_uuid() {
        let uuid = Uuid::from_str("97780ca3-a626-4fc5-b150-7fa8bc665df6").unwrap();
        let id = id!("97780ca3-a626-4fc5-b150-7fa8bc665df6");
        let converted_uuid: Uuid = id.into();
        assert_eq!(converted_uuid, uuid);
    }
}

#[derive(
    Encode,
    Debug,
    Decode,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
    Default
)]
pub struct PrincipalBytes([u8; Principal::MAX_LENGTH_IN_BYTES]);

impl PrincipalBytes {
    pub fn new(principal: Principal) -> Self {
        let mut bytes = [0; Principal::MAX_LENGTH_IN_BYTES];
        let principal_bytes = principal.as_slice();
        bytes[..principal_bytes.len()].copy_from_slice(principal_bytes);
        Self(bytes)
    }
}

impl From<Principal> for PrincipalBytes {
    fn from(principal: Principal) -> Self {
        Self::new(principal)
    }
}

impl From<PrincipalBytes> for Principal {
    fn from(principal_bytes: PrincipalBytes) -> Self {
        Principal::from_slice(&principal_bytes.0)
    }
}

impl CandidType for PrincipalBytes {
    fn _ty() -> candid::types::Type {
        <Principal as CandidType>::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
        where S: candid::types::Serializer
    {
        <Principal as CandidType>::idl_serialize(&Principal::from_slice(&self.0), serializer)
    }
}

#[cfg(test)]
mod principal_bytes_tests {
    use super::*;

    #[test]
    fn test_principal_bytes_conv() {
        let principal = Principal::anonymous();
        let principal_bytes = PrincipalBytes::new(principal);

        let principal = Into::<Principal>::into(principal_bytes);
    }
}

pub mod guard {
    use ic_principal::Principal;


    /// doesn't allow calls from anonymous principal
    pub fn verified_caller() -> Result<Principal, String> {
        let caller = ic_cdk::caller();

        if caller.ne(&Principal::anonymous()) {
            return Err(String::from("anonymous caller is not allowed"));
        }

        Ok(caller)
    }
}
