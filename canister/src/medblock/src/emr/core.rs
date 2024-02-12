use std::fmt::Debug;

use ic_stable_memory::OutOfMemory;
use ic_stable_structures::{ storable::Bound, BTreeMap};
use parity_scale_codec::{ Decode, Encode };

use crate::{
    impl_max_size,
    internal_types::{ AsciiRecordsKey, Id },
    mem::shared::{ MemBoundMarker, Memory, Stable },
};

type UserId = Id;
type ProviderId = Id;
type EmrId = Id;
type RecordsKey = AsciiRecordsKey;
type ArbitraryEmrValue = String;

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

    pub fn range_compose() -> Self {
        todo!()
    }
}

impl_max_size!(for CompositeKey: UserId, ProviderId, EmrId, RecordsKey);

impl MemBoundMarker for CompositeKey {
    const BOUND: Bound = Bound::Bounded { max_size: Self::max_size() as u32, is_fixed_size: false };
}

pub struct CoreRegistry(BTreeMap<Stable<CompositeKey>, ArbitraryEmrValue, Memory>);

impl Debug for CoreRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("CoreRegistry");

        for (key, value) in self.0.iter() {
            result.field(&format!("{:?}", key), &format!("{:?}", value));
        }

        result.finish()
    }
}

impl CoreRegistry {
    pub fn add(&mut self, key: Stable<CompositeKey>, value: ArbitraryEmrValue){
        self.0.insert(key, value);
    }

    pub fn update(&mut self, key: Stable<CompositeKey>, value: ArbitraryEmrValue) -> Result<(), OutOfMemory> {
        if let Some(_) = self.0.insert(key, value) {
            Ok(())
        } else {
            Err(OutOfMemory)
        }
    }

    // update a batch of records at once 
    pub fn update_batch(&mut self, records: Vec<(Stable<CompositeKey>, ArbitraryEmrValue)>) -> Result<(), OutOfMemory> {
        for (key, value) in records {
            if let Some(_) = self.0.insert(key, value) {
                return Err(OutOfMemory);
            }
        }

        Ok(())
    }

    pub fn remove_record(&mut self, key: Stable<CompositeKey>) -> bool {
        self.0.remove(&key).is_some()
    }

    pub fn get_list_provider_batch(_page: u64, _limit: u64) -> Vec<EmrId> {
        todo!()
    }

    pub fn get_list_user_batch(_page: u64, _limit: u64) -> Vec<EmrId> {
        todo!()
    }

    /** TODO: make type for conversing from opaque emr key-value type to type that can be seriaized */
    pub fn get_record_id() {
        todo!()
    }
}

// TODO: implement log for core registry
pub struct ActivityLog {
    user_id: UserId,
    provider_id: ProviderId,
    emr_id: EmrId,
    records_key: RecordsKey,
    add_activities: Vec<(RecordsKey, ArbitraryEmrValue)>,
    update_activities: Vec<(RecordsKey, ArbitraryEmrValue)>,
    remove_batch: Vec<RecordsKey>,
}

impl ActivityLog {
    pub fn new(
        user_id: UserId,
        provider_id: ProviderId,
        emr_id: EmrId,
        records_key: RecordsKey
    ) -> Self {
        Self {
            user_id,
            provider_id,
            emr_id,
            records_key,
            add_activities: Vec::new(),
            update_activities: Vec::new(),
            remove_batch: Vec::new(),
        }
    }

    pub fn add(&mut self, key: RecordsKey, value: ArbitraryEmrValue) {
        self.add_activities.push((key, value));
    }

    pub fn update(&mut self, key: RecordsKey, value: ArbitraryEmrValue) {
        self.update_activities.push((key, value));
    }

    pub fn update_batch(&mut self, records: Vec<(RecordsKey, ArbitraryEmrValue)>) {
        self.update_activities.extend(records);
    }

    pub fn remove_batch(&mut self, keys: Vec<RecordsKey>) {
        self.remove_batch.extend(keys);
    }
}