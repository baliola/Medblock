use candid::{ CandidType, Principal };
use ic_stable_memory::{
    collections::{ SBTreeMap, SBTreeSet },
    derive::{ AsFixedSizeBytes, StableType },
    primitive::s_ref::SRef,
};

use crate::{ deref, types::Id };

use super::OutOfMemory;

type EmrId = Id;
const KEY_LEN: usize = 32;

/// hex encoded SHA3-256 hash of NIK, used as key for [BindingMap].
/// we can't check for hash validity, so we assume it's valid by checking it's length.
#[derive(StableType, AsFixedSizeBytes, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct InternalBindingKey([u8; KEY_LEN]);

impl InternalBindingKey {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("key must be ascii")
    }
}

deref!(InternalBindingKey: [u8; KEY_LEN]);

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

pub type Owner = Principal;
pub type NIK = InternalBindingKey;
/// Principal to NIK Map. meant to enforce 1:1 relationship between principal and NIK.
/// used to claim emrs ownership. This level of inderction is needed because principal that map to a particular BindingKey effectively owns
/// all the emrs that it's BindingKey map to.
#[derive(Default)]
pub struct OwnerMap(SBTreeMap<Owner, NIK>);

impl OwnerMap {
    pub fn revoke(&mut self, owner: &Owner) {
        self.0.remove(owner);
    }

    pub fn bind(&mut self, owner: Owner, nik: NIK) -> Result<(), OutOfMemory> {
        self.0
            .insert(owner, nik)
            .map_err(OutOfMemory::from)
            .map(|_| ())
    }

    pub fn get_nik(&self, owner: &Owner) -> Option<SRef<'_, NIK>> {
        self.0.get(owner)
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_valid_owner(&self, owner: &Owner) -> bool {
        self.0.contains_key(owner)
    }
}

deref!(mut OwnerMap: SBTreeMap<Owner, NIK>);

pub type EmrIdCollection = SBTreeSet<EmrId>;
/// track emr issued for a particular user by storing it's emr id in this map. also used as blind index for emr search.
/// we use hashed (SHA3-256) NIK as key and emr id as value.
///
/// we don't use the principal directly because we want users to be able to change it's internet identity
/// and still be able to own and access their emr.
///
/// NIK MUST be hashed offchain before being used as key.
#[derive(Default)]
pub struct EmrBindingMap(SBTreeMap<NIK, EmrIdCollection>);

deref!(mut EmrBindingMap: SBTreeMap<NIK, EmrIdCollection>);

impl EmrBindingMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_owner_of(&self, nik: &NIK, emr_id: &EmrId) -> bool {
        self.0
            .get(nik)
            .map(|emr_ids| emr_ids.contains(emr_id))
            .unwrap_or(false)
    }

    pub fn issue_for(&mut self, nik: &NIK, emr_id: EmrId) -> Result<(), OutOfMemory> {
        if !self.0.contains_key(nik) {
            let issue_map = EmrIdCollection::new();
            self.0.insert(nik.clone(), issue_map);
        }

        let mut issue_map = self.0.get_mut(nik).unwrap();

        issue_map
            .insert(emr_id)
            .map_err(OutOfMemory::from)
            .map(|_| ())
    }
}
