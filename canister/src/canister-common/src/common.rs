use std::{ str::FromStr };

use candid::{ CandidType, Principal };
use ic_stable_structures::{ storable::Bound };
use parity_scale_codec::{ Decode, Encode };

use crate::{
    deref,
    from,
    impl_max_size,
    impl_mem_bound,
    impl_range_bound,
    mmgr::MemoryManager,
    stable::MemBoundMarker,
};
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

pub type UserId = H256;
pub type ProviderId = Id;
pub type EmrId = Id;
pub type RecordsKey<const N: usize> = AsciiRecordsKey<N>;
pub type ArbitraryEmrValue = String;

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

impl<const N: usize> TryFrom<&str> for AsciiRecordsKey<N> {
    type Error = AsciiKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<const N: usize> MemBoundMarker for AsciiRecordsKey<N> {
    const BOUND: Bound = Bound::Bounded { max_size: Self::max_size() as u32, is_fixed_size: false };
}

impl<const N: usize> AsciiRecordsKey<N> {
    pub const fn max_size() -> usize {
        N + 1
    }
}

impl<const N: usize> Default for AsciiRecordsKey<N> {
    fn default() -> Self {
        let mut key = [0_u8; N];
        key.fill(0);

        Self { key, len: Default::default() }
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

#[cfg(test)]
mod test_ascii_records {
    use super::*;

    #[test]
    fn test_len_encoded() {
        use candid::{ Encode, Decode };

        let key = AsciiRecordsKey::<64>::new("a".repeat(64)).unwrap();
        let encoded = Encode!(&key).unwrap();

        let decoded = Decode!(&encoded, AsciiRecordsKey::<64>).unwrap();

        assert_eq!(decoded, key);
    }
}

/// wrapper for [uuid::Uuid] because candid is not implemented for [uuid::Uuid]
#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Encode, Decode)]
pub struct Id([u8; 16]);
impl_max_size!(for Id: 16);
impl_mem_bound!(for Id: bounded; fixed_size: true);
impl_range_bound!(Id);

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

impl FromStr for Id {
    type Err = <Uuid as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(|s| s.into())
    }
}
impl TryInto<Id> for String {
    type Error = <Uuid as FromStr>::Err;

    fn try_into(self) -> Result<Id, Self::Error> {
        Uuid::from_str(&self).map(|s| s.into())
    }
}

impl CandidType for Id {
    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
        where S: candid::types::Serializer
    {
        serializer.serialize_text(self.to_string().as_str())
    }

    fn _ty() -> candid::types::Type {
        candid::types::TypeInner::Text.into()
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        Uuid::from_bytes_ref(&self.0).to_string()
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

            if s.len() > N {
                return Err(serde::de::Error::custom(format!("key exceeded max length of {}", N)));
            }

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

    impl<'de, const N: usize> serde::Serialize for AsciiRecordsKey<N> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
            let ref_str = self.to_ascii_str();

            serializer.serialize_str(ref_str)
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

        let _timestamp2 = Timestamp::new();
        let _now2 = SystemTime::now()
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
        let _uuid = Uuid::from_str("97780ca3-a626-4fc5-b150-7fa8bc665df6").unwrap();
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

    pub fn from_principal(principal: Principal) -> Self {
        Self::new(principal)
    }

    pub fn to_principal(self) -> Principal {
        self.into()
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

        let _principal = Into::<Principal>::into(principal_bytes);
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

pub mod freeze {
    use candid::CandidType;
    use ic_stable_structures::{ memory_manager::MemoryId, Cell };
    use serde::Deserialize;

    use crate::{
        impl_max_size,
        impl_mem_bound,
        mmgr::MemoryManager,
        stable::{ Candid, Memory, Stable, ToStable },
    };

    use super::Get;

    pub enum AllowCallFlag {
        Enabled,
        Disabled,
    }

    #[derive(CandidType, Deserialize)]
    pub struct FreezeThreshold {
        threshold: u128,
    }

    impl_max_size!(for FreezeThreshold: u128);
    impl_mem_bound!(for FreezeThreshold: bounded; fixed_size: false);

    impl FreezeThreshold {
        pub fn init<M: Get<MemoryId>>(
            threshold: u128,
            memory_manager: &MemoryManager
        ) -> Cell<Stable<Self, Candid>, Memory> {
            // safe to unwrap, we're using layout version 1
            memory_manager
                .get_memory::<_, M>(|m| Cell::init(m, Self::new(threshold).to_stable()))
                .unwrap()
        }
    }

    impl FreezeThreshold {
        pub fn new(threshold: u128) -> Self {
            Self {
                threshold,
            }
        }

        pub fn update_threshold(&mut self, threshold: u128) {
            self.threshold = threshold;
        }

        /// INSPECT MESSAGE, DO NOT ALLOW CALL TO BE ACCEPTED IF CANISTER BALANCE IS BELOW THRESHOLD
        ///
        /// MAKE SURE TO CALL THIS IN THE CANISTER INSPECT MESSAGE HANDLE
        pub fn check(&self) {
            let balance = ic_cdk::api::canister_balance128();
            match balance.cmp(&self.threshold) {
                std::cmp::Ordering::Less => ic_cdk::trap("canister is currently freezed"),
                _ => (),
            }
        }
    }
}

pub const HASH_LEN: usize = 32;

/// generap purpose 256 bit arbitrary hex encoded hash, could be used as index.
/// currently we make no assumption of the hash method used, as this should generally be
/// generated offchain and only deserialized onchain.
#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Encode, Decode)]
pub struct H256([u8; HASH_LEN]);

impl Default for H256 {
    fn default() -> Self {
        let mut buf = [0u8; HASH_LEN];
        buf.fill(0);

        Self(buf)
    }
}

impl_max_size!(for H256: 32);
impl_mem_bound!(for H256: bounded; fixed_size: true);
deref!(H256: [u8; HASH_LEN]);
impl_range_bound!(H256);
from!(H256: [u8; HASH_LEN]);

impl H256 {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("key must be ascii")
    }
}

impl ToString for H256 {
    fn to_string(&self) -> String {
        hex::encode(self.0)
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum H256Error {
    #[error(transparent)] HexError(#[from] hex::FromHexError),

    #[error("invalid nik hash length")]
    InvalidLength,
}
impl FromStr for H256 {
    type Err = H256Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = hex::decode(s)?;

        if s.len() != HASH_LEN {
            return Err(H256Error::InvalidLength);
        }

        let mut key = [0u8; HASH_LEN];
        key.copy_from_slice(&s);

        Ok(Self(key))
    }
}

mod deserialize_h256 {
    use super::*;

    impl<'de> serde::Deserialize<'de> for H256 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>
        {
            let s = String::deserialize(deserializer).map_err(|e|
                serde::de::Error::custom(
                    format!("error deserializing h256 from string with message  : {e}")
                )
            )?;
            Self::from_str(&s).map_err(serde::de::Error::custom)
        }
    }

    impl CandidType for H256 {
        fn _ty() -> candid::types::Type {
            candid::types::TypeInner::Text.into()
        }

        fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
            where S: candid::types::Serializer
        {
            serializer.serialize_text(self.as_str())
        }
    }

    #[cfg(test)]
    mod deserialize_test {
        use ic_cdk::println;
        use serde_assert::Token;
        use tiny_keccak::Hasher;

        use super::*;

        fn dummy_hash() -> String {
            let mut out = [0_u8; 32];
            let mut hasher = tiny_keccak::Keccak::v512();
            hasher.update("1".repeat(15).as_bytes());
            hasher.finalize(&mut out);

            hex::encode(out)
        }

        #[test]
        fn test_from_str() {
            let hash = dummy_hash();
            let h256 = H256::from_str(&hash).unwrap();

            assert_eq!(h256.to_string(), hash);
        }

        #[test]
        fn test_deserialize_h256() {
            let hash = "9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c";
            let h256 = H256::from_str(hash).unwrap();

            let h256_str = h256.to_string();
            println!("h256_str : {}", h256_str);

            let mut deserializer = serde_assert::Deserializer
                ::builder([Token::Str(hash.to_string())])
                .build();

            let h256_deserialized = H256::deserialize(&mut deserializer).unwrap();

            assert_eq!(h256, h256_deserialized);
        }

        #[test]
        fn test_wrong_hash() {
            let hash = "51e04ecd372fbbd123dd842bc485b87db5c2a50e4ea83590108363c56ae38d";
            let h256 = H256::from_str(hash);

            let Err(e) = h256 else { unreachable!() };
            assert_eq!(e, H256Error::InvalidLength);

            let mut deserializer = serde_assert::Deserializer
                ::builder([Token::Str(hash.to_string())])
                .build();

            let h256_deserialized = H256::deserialize(&mut deserializer);
            let Err(e) = h256_deserialized else { unreachable!() };

            assert_eq!(e.to_string(), H256Error::InvalidLength.to_string());
        }
    }
}

#[derive(Debug, Deserialize, Clone, CandidType, PartialEq, Eq)]
pub struct EmrFragment {
    pub key: AsciiRecordsKey,
    pub value: ArbitraryEmrValue,
}

impl EmrFragment {
    pub fn new(key: AsciiRecordsKey, value: ArbitraryEmrValue) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Deserialize, Clone, CandidType, PartialEq, Eq)]
pub struct EmrBody(Vec<EmrFragment>);

impl EmrBody {
    pub fn into_inner(self) -> Vec<EmrFragment> {
        self.0
    }
}

impl From<Vec<EmrFragment>> for EmrBody {
    fn from(records: Vec<EmrFragment>) -> Self {
        Self(records)
    }
}

impl From<Vec<(AsciiRecordsKey, ArbitraryEmrValue)>> for EmrBody {
    fn from(records: Vec<(AsciiRecordsKey, ArbitraryEmrValue)>) -> Self {
        let records = records
            .into_iter()
            .map(|(k, v)| EmrFragment::new(k, v))
            .collect::<Vec<_>>();

        Self(records)
    }
}

impl IntoIterator for EmrBody {
    type Item = EmrFragment;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
        // .map(|fragment| (fragment.key, fragment.value))
        // .collect::<Vec<_>>()
        // .into_iter()
    }
}

pub struct State<Registry, Config, Threshold> {
    pub registry: Registry,
    pub config: Config,
    pub freeze_threshold: Threshold,
    pub memory_manager: MemoryManager,
}

impl<Registry, Config, Threshold> State<Registry, Config, Threshold> {
    pub fn new(
        registry: Registry,
        config: Config,
        freeze_threshold: Threshold,
        memory_manager: MemoryManager
    ) -> Self {
        Self { registry, config, freeze_threshold, memory_manager }
    }
}

pub trait Get<T> {
    fn get() -> T;
}

#[derive(
    Debug,
    Deserialize,
    CandidType,
    PartialEq,
    Eq,
    Clone,
    PartialOrd,
    Ord,
    Default,
    Encode,
    Decode
)]
pub struct EmrHeader {
    pub emr_id: EmrId,
    pub provider_id: ProviderId,
    pub user_id: UserId,
    /// reserved for future use of multiple emr registry canister
    pub registry_id: PrincipalBytes,
}


impl_max_size!(for EmrHeader: EmrHeader);
impl_mem_bound!(for EmrHeader: bounded; fixed_size: true);
impl_range_bound!(EmrHeader);

impl EmrHeader {
    pub fn new(
        user_id: UserId,
        emr_id: EmrId,
        provider_id: ProviderId,
        registry_id: Principal
    ) -> Self {
        Self { user_id, emr_id, provider_id, registry_id: PrincipalBytes::from(registry_id) }
    }
}

#[cfg(test)]
mod header_test {
    use super::*;

    #[test]
    fn test_len_encoded() {
        

        let header = EmrHeader::new(
            UserId::default(),
            EmrId::default(),
            ProviderId::default(),
            Principal::anonymous()
        );

        let encoded = header.encode();
        println!("{:?}", encoded.len());

        let decoded = <EmrHeader as parity_scale_codec::Decode>::decode(&mut &*encoded).unwrap();

        assert_eq!(decoded, header);
    }
}

#[derive(Debug, Deserialize, CandidType, PartialEq, Eq)]
pub struct EmrHeaderWithBody {
    pub header: EmrHeader,
    pub body: EmrBody,
}

impl EmrHeaderWithBody {
    pub fn new(header: EmrHeader, body: EmrBody) -> Self {
        Self { header, body }
    }

    pub fn to_header(self) -> EmrHeader {
        self.header
    }

    pub fn into_inner_body(self) -> EmrBody {
        self.body
    }
}

/// get canister id, will return [Principal::anonymous] if not running on the ic
pub fn canister_id() -> Principal {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Principal::anonymous()
    }

    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::api::id()
    }
}
