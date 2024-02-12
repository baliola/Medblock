use std::fmt::Debug;

use ic_stable_memory::OutOfMemory;
use ic_stable_structures::{ storable::Bound, BTreeMap, Log };
use parity_scale_codec::{ Decode, Encode };

use crate::{
    impl_max_size,
    internal_types::{ AsciiRecordsKey, Id },
    mem::shared::{ MemBoundMarker, Memory, Stable },
};

use super::Emr;

pub type UserId = Id;
pub type ProviderId = Id;
pub type EmrId = Id;
pub type RecordsKey = AsciiRecordsKey;
pub type ArbitraryEmrValue = String;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub struct CompositeKey(UserId, ProviderId, EmrId, RecordsKey);

impl CompositeKey {
    pub fn new(
        user_id: UserId,
        provider_id: ProviderId,
        emr_id: EmrId,
        records_key: RecordsKey
    ) -> Self {
        Self(user_id, provider_id, emr_id, records_key)
    }
}

impl_max_size!(for CompositeKey: UserId, ProviderId, EmrId, RecordsKey);

impl MemBoundMarker for CompositeKey {
    const BOUND: Bound = Bound::Bounded { max_size: Self::max_size() as u32, is_fixed_size: false };
}

// ----------------------------------------- Begin Builder -----------------------------------------

pub struct CompositeKeyBuilder {
    user_id: Option<UserId>,
    provider_id: Option<ProviderId>,
    emr_id: Option<EmrId>,
    records_key: Option<RecordsKey>,
}

impl CompositeKeyBuilder {
    pub fn with_user(&mut self, user: UserId) -> &mut Self {
        self.user_id = Some(user);
        self
    }

    pub fn with_provider(&mut self, provider: ProviderId) -> &mut Self {
        self.provider_id = Some(provider);
        self
    }

    pub fn with_emr_id(&mut self, emr_id: EmrId) -> &mut Self {
        self.emr_id = Some(emr_id);
        self
    }

    pub fn records_key(&mut self, records_key: RecordsKey) -> &mut Self {
        self.records_key = Some(records_key);
        self
    }

    pub fn build(self) -> CompositeKey {
        let user_id = self.user_id.unwrap_or_default();
        let provider_id = self.provider_id.unwrap_or_default();
        let emr_id = self.emr_id.unwrap_or_default();
        let records_key = self.records_key.unwrap_or_default();

        CompositeKey::new(user_id, provider_id, emr_id, records_key)
    }
}

// ----------------------------------------- End   Builder -----------------------------------------
