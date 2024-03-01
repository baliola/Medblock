pub mod patient;
pub mod providers;
pub mod core;
pub mod key;

use candid::{ CandidType, Principal };
use ic_stable_memory::{
    collections::SHashMap,
    derive::{ AsFixedSizeBytes, StableType },
    primitive::{ s_ref::SRef, s_ref_mut::SRefMut },
    SBox,
    StableType,
};
use serde::Deserialize;
use serde_json::Value;

/// marker for types that can be serialized as response, it basically have 2 requirements
/// and that is candid type and cloneable. this works because while stable memory type may implement
/// candid, it cannot implement clone 'safely' as cloning a stable memory data involves
/// allocating stable memory in which it may fail due to memory exhaustion.
pub trait ResponseMarker: CandidType + Clone + FromStableRef {}

/// this basically enforce that response maker type is only be able to be created from stable memory reference, effectively mirroring the stable memory data to heap
pub trait FromStableRef {
    type From: StableType;

    fn from_stable_ref(sref: &Self::From) -> Self;
}

pub trait ToResponse<T: ResponseMarker> {
    fn to_response(&self) -> T;
}

use crate::{
    deref,
    internal_types::{ AsciiRecordsKey, Id, Timestamp },
    measure_alloc,
    mem::shared::Stable,
};

use self::{
    core::{ CoreEmrRegistry, RawEmr },
    key::{ CompositeKeyBuilder, ProviderId, UserId },
    patient::{ EmrBindingMap, InternalBindingKey, OwnerMap, NIK },
};

#[derive(Debug, thiserror::Error, CandidType, serde::Deserialize)]
pub enum RegistryError {
    #[error(transparent)] OwnerMapError(#[from] patient::PatientBindingMapError),
    #[error(transparent)] CoreRegistryError(#[from] core::CoreRegistryError),
}

pub type RegistryResult<T = ()> = Result<T, RegistryError>;

pub struct EmrRegistry {
    owners: OwnerMap,
    owner_emrs: EmrBindingMap,
    core_emrs: CoreEmrRegistry,
}

// placeholder, will be removed later
impl Default for EmrRegistry {
    fn default() -> Self {
        todo!()
    }
}

impl EmrRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// register new emr to the system.
    pub fn register_emr(
        &mut self,
        user_id: InternalBindingKey,
        provider: ProviderId,
        emr_id: EmrId,
        emr_records: RawEmr
    ) -> EmrId {
        let key = key::CompositeKeyBuilder
            ::new()
            .records_key()
            .with_user(user_id.clone())
            .with_provider(provider)
            .with_emr_id(emr_id.clone());

        self.core_emrs.add(key, emr_records);
        self.owner_emrs.issue_for(user_id, emr_id.clone());

        emr_id
    }

    pub fn register_patient(
        &mut self,
        owner: ic_principal::Principal,
        hashed_nik: NIK
    ) -> RegistryResult {
        Ok(self.owners.bind(owner, hashed_nik)?)
    }

    pub fn rebind_patient(
        &mut self,
        owner: ic_principal::Principal,
        hashed_nik: NIK
    ) -> RegistryResult {
        Ok(self.owners.rebind(owner, hashed_nik)?)
    }

    /// revoke patient access, if this method is called then the patient will no longer be able to access their emr. it will remove the [NIK]
    /// from the owner map so attempting to access NIK owner will fail.
    pub fn revoke_patient_access(&mut self, owner: &ic_principal::Principal) -> RegistryResult {
        Ok(self.owners.revoke(owner)?)
    }

    pub fn is_owner_of_emr(&self, owner: &ic_principal::Principal, emr_id: EmrId) -> bool {
        let Ok(nik) = self.owners.get_nik(owner) else {
            return false;
        };

        self.owner_emrs.is_owner_of(nik.into_inner(), emr_id)
    }

    pub fn update_emr(
        &mut self,
        emr: RawEmr,
        user_id: UserId,
        provider: ProviderId,
        emr_id: Id
    ) -> RegistryResult {
        let key = CompositeKeyBuilder::new()
            .emr()
            .with_user(user_id.clone())
            .with_provider(provider.clone())
            .with_emr_id(emr_id.clone());

        self.core_emrs.is_emr_exists(key)?;

        let partial_key = CompositeKeyBuilder::new()
            .records_key()
            .with_user(user_id)
            .with_provider(provider)
            .with_emr_id(emr_id);

        for (k, v) in emr.into_iter() {
            let key = partial_key.clone().with_records_key(k);
            self.core_emrs.update(key, v);
        }

        Ok(())
    }

    /// get all user emr id, will return [None] if the nik used as index is invalid or no emr was found
    pub fn get_patient_emr_list(
        &self,
        patient: &patient::Owner
    ) -> RegistryResult<Vec<Stable<EmrId>>> {
        let internal_id = self.owners.get_nik(patient)?;
        Ok(self.owner_emrs.emr_list(&internal_id)?)
    }

    pub fn is_valid_patient(&self, owner: &patient::Owner) -> bool {
        self.owners.is_valid_owner(owner)
    }

    pub fn get_emr(
        &self,
        user_id: &ic_principal::Principal,
        provider: ProviderId,
        emr_id: Id
    ) -> RegistryResult<RawEmr> {
        let user_id = self.owners.get_nik(&user_id)?.into_inner();
        let key = CompositeKeyBuilder::new()
            .emr()
            .with_user(user_id)
            .with_provider(provider)
            .with_emr_id(emr_id);

        Ok(self.core_emrs.read_by_id(key)?)
    }
}

type EmrId = Id;

