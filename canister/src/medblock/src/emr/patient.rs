use candid::{ CandidType, Principal };
use ic_stable_memory::{
    collections::{ SBTreeMap, SBTreeSet },
    derive::{ AsFixedSizeBytes, StableType },
    primitive::s_ref::SRef,
};
use ic_stable_structures::memory_manager;
use parity_scale_codec::{ Decode, Encode };
use serde_json::error;

use crate::{
    deref,
    impl_max_size,
    impl_mem_bound,
    impl_range_bound,
    internal_types::Id,
    mem::{ shared::{ Memory, Stable, StableSet, ToStable }, MemoryManager },
};

use super::{ key::UserId, OutOfMemory };

type EmrId = Id;
const KEY_LEN: usize = 32;

/// hex encoded SHA3-256 hash of NIK, used as key for [BindingMap].
/// we can't check for hash validity, so we assume it's valid by checking it's length.
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
    Encode,
    Decode,
    Default
)]
pub struct InternalBindingKey([u8; KEY_LEN]);
impl_max_size!(for InternalBindingKey: 32);
impl_mem_bound!(for InternalBindingKey: bounded; fixed_size: true);
deref!(InternalBindingKey: [u8; KEY_LEN]);
impl_range_bound!(InternalBindingKey);

impl InternalBindingKey {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("key must be ascii")
    }
}

mod deserialize {
    use super::*;

    impl<'de> serde::Deserialize<'de> for InternalBindingKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>
        {
            let s = String::deserialize(deserializer)?;
            let s = hex::decode(s).map_err(serde::de::Error::custom)?;

            if s.len() != KEY_LEN {
                return Err(serde::de::Error::custom("invalid nik hash length"));
            }

            // TODO: unnecessary copy
            let mut key = [0u8; KEY_LEN];
            key[..s.len()].copy_from_slice(&s);

            Ok(Self(key))
        }
    }

    impl CandidType for InternalBindingKey {
        fn _ty() -> candid::types::Type {
            candid::types::Type::Text
        }

        fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
            where S: candid::types::Serializer
        {
            serializer.serialize_text(self.as_str())
        }
    }
}

pub type NIK = InternalBindingKey;
/// Principal to NIK Map. meant to enforce 1:1 relationship between principal and NIK.
/// used to claim emrs ownership. This level of inderction is needed because principal that map to a particular BindingKey effectively owns
/// all the emrs that it's BindingKey map to.
pub type Owner = ic_principal::Principal;
pub struct OwnerMap(ic_stable_structures::BTreeMap<Owner, Stable<NIK>, Memory>);

#[derive(Debug, thiserror::Error, CandidType, serde::Deserialize)]
pub enum OwnerMapError {
    #[error("operation not permitted, user exists")]
    UserExist,
    #[error("operation not permitted, user does not exist")]
    UserDoesNotExist,
}

impl OwnerMap {
    pub fn revoke(&mut self, owner: &Owner) {
        self.0.remove(owner);
    }

    pub fn bind(&mut self, owner: Owner, nik: NIK) -> Result<(), OwnerMapError> {
        if self.get_nik(&owner).is_ok() {
            return Err(OwnerMapError::UserExist);
        }

        let _ = self.0.insert(owner, nik.to_stable());
        Ok(())
    }

    /// will return an error if owner does not exists
    pub fn get_nik(&self, owner: &Owner) -> Result<Stable<UserId>, OwnerMapError> {
        self.0.get(owner).ok_or(OwnerMapError::UserDoesNotExist)
    }

    pub fn new(memory_manager: MemoryManager) -> Self {
        Self(memory_manager.get_memory(ic_stable_structures::BTreeMap::new))
    }

    pub fn is_valid_owner(&self, owner: &Owner) -> bool {
        self.0.contains_key(owner)
    }
}

pub type EmrIdCollection = SBTreeSet<EmrId>;
/// track emr issued for a particular user by storing it's emr id in this map. also used as blind index for emr search.
/// we use hashed (SHA3-256) NIK as key and emr id as value.
///
/// we don't use the principal directly because we want users to be able to change it's internet identity
/// and still be able to own and access their emr.
///
/// NIK MUST be hashed offchain before being used as key.
pub struct EmrBindingMap(StableSet<Stable<NIK>, Stable<Id>>);

impl EmrBindingMap {
    pub fn new(memory_manager: MemoryManager) -> Self {
        Self(StableSet::new(memory_manager))
    }

    pub fn is_owner_of(&self, nik: NIK, emr_id: EmrId) -> bool {
        self.0.contains_key(nik.to_stable(), emr_id.to_stable())
    }

    pub fn emr_list(&self, nik: &NIK) -> Option<Vec<EmrId>> {
        let list = self.0.get_set_associated_by_key(&nik.clone().to_stable())?;

        Some(
            list
                .into_iter()
                .map(|id| id.into_inner())
                .collect()
        )
    }

    pub fn issue_for(&mut self, nik: NIK, emr_id: EmrId) {
        let _ = self.0.insert(nik.to_stable(), emr_id.to_stable());
    }
}
