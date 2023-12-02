use candid::{CandidType, Principal};
use ic_stable_memory::{
    collections::{SBTreeMap, SBTreeSet, SVec},
    derive::{AsFixedSizeBytes, StableType},
    AsFixedSizeBytes, StableType,
};

use crate::{deref, types::Id};

type Owner = Principal;
type NIK = BindingKey;
/// Principal to NIK Map. meant to enforce 1:1 relationship between principal and NIK.
/// used to claim emrs ownership. This level of inderction is needed because principal that map to a particular BindingKey effectively owns
/// all the emrs that it's BindingKey map to.
pub struct OwnerMap(SBTreeMap<Owner, NIK>);

type EmrCollection = SBTreeSet<EmrId>;
/// track emr issued for a particular user by storing it's emr id in this map. also used as blind index for emr search.
/// we use hashed (keccak256) NIK as key and emr id as value.
///
/// we don't use the principal directly because we want users to be able to change it's internet identity
/// and still be able to own and access their emr.
///
/// NIK SHOULD be hashed offchain before being used as key.
pub struct EmrBindingMap(SBTreeMap<BindingKey, EmrCollection>);
deref!(EmrBindingMap: SBTreeMap<BindingKey, EmrCollection>);

impl EmrBindingMap {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for EmrBindingMap {
    fn default() -> Self {
        Self(SBTreeMap::new())
    }
}

type EmrId = Id;
const KEY_LEN: usize = 32;

/// SHA3-256 hash of NIK, used as key for [BindingMap].
/// we can't check for hash validity, so we assume it's valid by checking it's length.
#[derive(
    StableType, AsFixedSizeBytes, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug, CandidType,
)]
pub struct BindingKey([u8; KEY_LEN]);
deref!(BindingKey: [u8; KEY_LEN]);

mod deserialize {
    use super::*;
    
    impl<'de> serde::Deserialize<'de> for BindingKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?.into_bytes();

            if s.len() != KEY_LEN {
                return Err(serde::de::Error::custom("invalid nik hash length"));
            }

            // TODO: unnecessary copy
            let mut key = [0u8; KEY_LEN];
            key[..s.len()].copy_from_slice(&s);

            Ok(Self(key))
        }
    }
}
