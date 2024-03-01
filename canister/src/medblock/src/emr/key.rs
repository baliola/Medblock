#![allow(unused)]

use std::{ fmt::Debug, marker::PhantomData, ops::RangeBounds };

use ic_stable_structures::{ storable::Bound };
use parity_scale_codec::{ Decode, Encode };

use crate::{
    impl_max_size,
    internal_types::{ AsciiRecordsKey, Id },
    mem::shared::{ MemBoundMarker },
    zero_sized_state,
};

use super::patient::NIK;

pub type UserId = NIK;
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

    pub fn builder() -> CompositeKeyBuilder<UnknownUsage> {
        CompositeKeyBuilder::<UnknownUsage>::new()
    }
}

impl_max_size!(for CompositeKey: UserId, ProviderId, EmrId, RecordsKey);

impl MemBoundMarker for CompositeKey {
    const BOUND: Bound = Bound::Bounded { max_size: Self::max_size() as u32, is_fixed_size: false };
}

// ----------------------------------------- Begin Builder -----------------------------------------

/// marker trait for usage
pub trait UsageMarker {}
zero_sized_state!(UserBatch, ProviderBatch, ByEmr, ByRecordsKey, UnknownUsage);

/// used to get the correct threshold for the composite key
pub trait Threshold {
    type T;

    /// get the correct threshold for this key, used to short circuit iteration on map and set to improve performance
    fn threshold(key: &CompositeKey) -> &Self::T where Self: Sized;
}

impl Threshold for UserBatch {
    type T = NIK;

    fn threshold(key: &CompositeKey) -> &NIK {
        key.user_id()
    }
}

impl Threshold for ProviderBatch {
    type T = ProviderId;

    fn threshold(key: &CompositeKey) -> &ProviderId {
        key.provider_id()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Unknown<T>(PhantomData<T>);

#[derive(Debug, Clone)]
pub struct Known<T>(T);

impl<T: Default> Default for Known<T> {
    fn default() -> Self {
        Self(T::default())
    }
}
impl<T> Known<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct CompositeKeyBuilder<
    Usage,
    UnknownUser = Unknown<UserId>,
    UnknownProvider = Unknown<ProviderId>,
    UnknownEmrId = Unknown<EmrId>,
    UnknownRecordsKey = Unknown<RecordsKey>
> {
    user_id: UnknownUser,
    provider_id: UnknownProvider,
    emr_id: UnknownEmrId,
    records_key: UnknownRecordsKey,
    __marker: std::marker::PhantomData<Usage>,
}

impl CompositeKeyBuilder<UserBatch> {
    pub fn with_user(self, user: UserId) -> CompositeKeyBuilder<UserBatch, Known<UserId>> {
        CompositeKeyBuilder {
            user_id: Known(user),
            ..Default::default()
        }
    }
}

impl CompositeKeyBuilder<UserBatch, Known<UserId>> {
    pub fn with_provider(
        self,
        provider: ProviderId
    ) -> CompositeKeyBuilder<UserBatch, Known<UserId>, Known<ProviderId>> {
        CompositeKeyBuilder {
            user_id: self.user_id,
            provider_id: Known(provider),
            ..Default::default()
        }
    }

    pub fn build(self) -> CompositeKey {
        let user_id = self.user_id.into_inner();
        let provider_id = ProviderId::default();
        let emr_id = EmrId::default();
        let records_key = AsciiRecordsKey::default();

        CompositeKey::new(user_id, provider_id, emr_id, records_key)
    }
}

impl CompositeKeyBuilder<UserBatch, Known<UserId>, Known<ProviderId>> {
    pub fn build(self) -> CompositeKey {
        let user_id = self.user_id.into_inner();
        let provider_id = self.provider_id.into_inner();

        let emr_id = EmrId::default();
        let records_key = AsciiRecordsKey::default();

        CompositeKey::new(user_id, provider_id, emr_id, records_key)
    }
}

impl CompositeKeyBuilder<ByEmr> {
    pub fn with_user(self, user: UserId) -> CompositeKeyBuilder<ByEmr, Known<UserId>> {
        CompositeKeyBuilder {
            user_id: Known(user),
            ..Default::default()
        }
    }
}

impl CompositeKeyBuilder<ByEmr, Known<UserId>> {
    pub fn with_provider(
        self,
        provider: ProviderId
    ) -> CompositeKeyBuilder<ByEmr, Known<UserId>, Known<ProviderId>> {
        CompositeKeyBuilder {
            user_id: self.user_id,
            provider_id: Known(provider),
            ..Default::default()
        }
    }
}
impl CompositeKeyBuilder<ByEmr, Known<UserId>, Known<ProviderId>> {
    pub fn with_emr_id(
        self,
        emr_id: EmrId
    ) -> CompositeKeyBuilder<ByEmr, Known<UserId>, Known<ProviderId>, Known<EmrId>> {
        CompositeKeyBuilder {
            user_id: self.user_id,
            provider_id: self.provider_id,
            emr_id: Known(emr_id),
            ..Default::default()
        }
    }
}

impl CompositeKeyBuilder<ByEmr, Known<UserId>, Known<ProviderId>, Known<EmrId>> {
    pub fn build(self) -> CompositeKey {
        let user_id = self.user_id.into_inner();
        let provider_id = self.provider_id.into_inner();
        let emr_id = self.emr_id.into_inner();
        let records_key = AsciiRecordsKey::default();

        CompositeKey::new(user_id, provider_id, emr_id, records_key)
    }
}

impl CompositeKeyBuilder<ProviderBatch> {
    pub fn with_provider(
        self,
        provider: ProviderId
    ) -> CompositeKeyBuilder<
        ProviderBatch,
        Unknown<UserId>,
        Known<ProviderId>,
        Unknown<EmrId>,
        Unknown<RecordsKey>
    > {
        CompositeKeyBuilder {
            provider_id: Known(provider),
            ..Default::default()
        }
    }
}

impl CompositeKeyBuilder<ProviderBatch, Unknown<UserId>, Known<ProviderId>> {
    pub fn build(self) -> CompositeKey {
        let user_id = UserId::default();
        let provider_id = self.provider_id.into_inner();
        let emr_id = EmrId::default();
        let records_key = AsciiRecordsKey::default();

        CompositeKey::new(user_id, provider_id, emr_id, records_key)
    }
}

impl CompositeKeyBuilder<ByRecordsKey> {
    pub fn with_user(self, user: UserId) -> CompositeKeyBuilder<ByRecordsKey, Known<UserId>> {
        CompositeKeyBuilder {
            user_id: Known(user),
            ..Default::default()
        }
    }
}

impl CompositeKeyBuilder<ByRecordsKey, Known<UserId>> {
    pub fn with_provider(
        self,
        provider: ProviderId
    ) -> CompositeKeyBuilder<ByRecordsKey, Known<UserId>, Known<ProviderId>> {
        CompositeKeyBuilder {
            user_id: self.user_id,
            provider_id: Known(provider),
            ..Default::default()
        }
    }
}

impl CompositeKeyBuilder<ByRecordsKey, Known<UserId>, Known<ProviderId>> {
    pub fn with_emr_id(
        self,
        emr_id: EmrId
    ) -> CompositeKeyBuilder<ByRecordsKey, Known<UserId>, Known<ProviderId>, Known<EmrId>> {
        CompositeKeyBuilder {
            user_id: self.user_id,
            provider_id: self.provider_id,
            emr_id: Known(emr_id),
            ..Default::default()
        }
    }
}

impl CompositeKeyBuilder<ByRecordsKey, Known<UserId>, Known<ProviderId>, Known<EmrId>> {
    pub fn with_records_key(
        self,
        records_key: RecordsKey
    ) -> CompositeKeyBuilder<
        ByRecordsKey,
        Known<UserId>,
        Known<ProviderId>,
        Known<EmrId>,
        Known<RecordsKey>
    > {
        CompositeKeyBuilder {
            user_id: self.user_id,
            provider_id: self.provider_id,
            emr_id: self.emr_id,
            records_key: Known(records_key),
            ..Default::default()
        }
    }
}

impl CompositeKeyBuilder<
    ByRecordsKey,
    Known<UserId>,
    Known<ProviderId>,
    Known<EmrId>,
    Known<RecordsKey>
> {
    pub fn build(self) -> CompositeKey {
        let user_id = self.user_id.into_inner();
        let provider_id = self.provider_id.into_inner();
        let emr_id = self.emr_id.into_inner();
        let records_key = self.records_key.into_inner();

        CompositeKey::new(user_id, provider_id, emr_id, records_key)
    }
}

impl CompositeKeyBuilder<UnknownUsage> {
    pub fn new() -> CompositeKeyBuilder<UnknownUsage> {
        CompositeKeyBuilder::<UnknownUsage>::default()
    }

    fn new_with_usage<Usage: Default>() -> CompositeKeyBuilder<Usage> {
        CompositeKeyBuilder::<Usage>::default()
    }

    pub fn user_batch(self) -> CompositeKeyBuilder<UserBatch> {
        Self::new_with_usage::<_>()
    }

    pub fn provider_batch(self) -> CompositeKeyBuilder<ProviderBatch> {
        Self::new_with_usage::<_>()
    }

    pub fn emr(self) -> CompositeKeyBuilder<ByEmr> {
        Self::new_with_usage::<_>()
    }

    pub fn records_key(self) -> CompositeKeyBuilder<ByRecordsKey> {
        Self::new_with_usage::<_>()
    }
}

// ----------------------------------------- End   Builder -----------------------------------------
