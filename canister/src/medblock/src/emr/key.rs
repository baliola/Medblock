use std::{ fmt::Debug, ops::RangeBounds };

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, Default)]
pub struct CompositeKey(UserId, ProviderId, EmrId, RecordsKey);

impl RangeBounds<CompositeKey> for CompositeKey {
    fn start_bound(&self) -> core::ops::Bound<&CompositeKey> {
        core::ops::Bound::Included(self)
    }

    fn end_bound(&self) -> core::ops::Bound<&CompositeKey> {
        core::ops::Bound::Unbounded
    }
}
impl CompositeKey {
    pub fn new(
        user_id: UserId,
        provider_id: ProviderId,
        emr_id: EmrId,
        records_key: RecordsKey
    ) -> Self {
        Self(user_id, provider_id, emr_id, records_key)
    }

    pub fn user_id(&self) -> &UserId {
        &self.0
    }

    pub fn provider_id(&self) -> &ProviderId {
        &self.1
    }

    pub fn emr_id(&self) -> &EmrId {
        &self.2
    }

    pub fn record_key(&self) -> &RecordsKey {
        &self.3
    }

    pub fn builder() -> CompositeKeyBuilder<UknownUsage> {
        CompositeKeyBuilder::<UknownUsage>::new()
    }
}

impl_max_size!(for CompositeKey: UserId, ProviderId, EmrId, RecordsKey);

impl MemBoundMarker for CompositeKey {
    const BOUND: Bound = Bound::Bounded { max_size: Self::max_size() as u32, is_fixed_size: false };
}

// ----------------------------------------- Begin Builder -----------------------------------------
pub struct UserBatch;
pub struct ProviderBatch;
pub struct ByEmr;
pub struct ByRecordsKey;

pub struct UknownUsage;

pub struct CompositeKeyBuilder<Usage> {
    user_id: Option<UserId>,
    provider_id: Option<ProviderId>,
    emr_id: Option<EmrId>,
    records_key: Option<RecordsKey>,
    __marker: std::marker::PhantomData<Usage>,
}

impl CompositeKeyBuilder<UserBatch> {
    pub fn with_provider(&mut self, provider: ProviderId) -> &mut Self {
        self.provider_id = Some(provider);
        self
    }

    pub fn with_user(&mut self, user: UserId) -> &mut Self {
        self.user_id = Some(user);
        self
    }

    pub fn build(self) -> CompositeKey {
        let user_id = self.user_id.expect("user_id is required");
        let provider_id = self.provider_id.expect("provider_id is required");
        let emr_id = EmrId::default();
        let records_key = AsciiRecordsKey::default();

        CompositeKey::new(user_id, provider_id, emr_id, records_key)
    }
}

impl CompositeKeyBuilder<ByEmr> {
    pub fn with_provider(&mut self, provider: ProviderId) -> &mut Self {
        self.provider_id = Some(provider);
        self
    }

    pub fn with_user(&mut self, user: UserId) -> &mut Self {
        self.user_id = Some(user);
        self
    }

    pub fn with_emr_id(&mut self, emr_id: EmrId) -> &mut Self {
        self.emr_id = Some(emr_id);
        self
    }

    pub fn build(self) -> CompositeKey {
        let user_id = self.user_id.expect("user_id is required");
        let provider_id = self.provider_id.expect("provider_id is required");
        let emr_id = self.emr_id.expect("emr_id is required");
        let records_key = AsciiRecordsKey::default();

        CompositeKey::new(user_id, provider_id, emr_id, records_key)
    }
}

impl CompositeKeyBuilder<ProviderBatch> {
    pub fn with_provider(&mut self, provider: ProviderId) -> &mut Self {
        self.provider_id = Some(provider);
        self
    }

    pub fn build(self) -> CompositeKey {
        let user_id = UserId::default();
        let provider_id = self.provider_id.expect("provider_id is required");
        let emr_id = EmrId::default();
        let records_key = AsciiRecordsKey::default();

        CompositeKey::new(user_id, provider_id, emr_id, records_key)
    }
}

impl CompositeKeyBuilder<ByRecordsKey> {
    pub fn with_provider(&mut self, provider: ProviderId) -> &mut Self {
        self.provider_id = Some(provider);
        self
    }

    pub fn with_user(&mut self, user: UserId) -> &mut Self {
        self.user_id = Some(user);
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
        let user_id = self.user_id.expect("user_id is required");
        let provider_id = self.provider_id.expect("provider_id is required");
        let emr_id = self.emr_id.expect("emr_id is required");
        let records_key = self.records_key.expect("records_key is required");

        CompositeKey::new(user_id, provider_id, emr_id, records_key)
    }
}

impl CompositeKeyBuilder<UknownUsage> {
    pub fn new() -> CompositeKeyBuilder<UknownUsage> {
        CompositeKeyBuilder {
            user_id: None,
            provider_id: None,
            emr_id: None,
            records_key: None,
            __marker: std::marker::PhantomData,
        }
    }
    fn new_with_usage<Usage>() -> CompositeKeyBuilder<Usage> {
        CompositeKeyBuilder {
            user_id: None,
            provider_id: None,
            emr_id: None,
            records_key: None,
            __marker: std::marker::PhantomData,
        }
    }

    pub fn user_batch(self) -> CompositeKeyBuilder<UserBatch> {
        Self::new_with_usage::<_>()
    }

    pub fn provider_batch(self) -> CompositeKeyBuilder<ProviderBatch> {
        Self::new_with_usage::<_>()
    }

    pub fn by_emr(self) -> CompositeKeyBuilder<ByEmr> {
        Self::new_with_usage::<_>()
    }

    pub fn by_records_key(self) -> CompositeKeyBuilder<ByRecordsKey> {
        Self::new_with_usage::<_>()
    }
}

// ----------------------------------------- End   Builder -----------------------------------------
