use std::fmt::Debug;

use ic_stable_memory::OutOfMemory;
use ic_stable_structures::{ storable::Bound, BTreeMap, Log };
use parity_scale_codec::{ Decode, Encode };

use crate::{
    impl_max_size,
    internal_types::{ AsciiRecordsKey, Id },
    mem::shared::{ MemBoundMarker, Memory, Stable },
};

use super::key::{ ArbitraryEmrValue, CompositeKey, EmrId };

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
    pub fn add(&mut self, key: Stable<CompositeKey>, value: ArbitraryEmrValue) {
        self.0.insert(key, value);
    }

    pub fn update(
        &mut self,
        key: Stable<CompositeKey>,
        value: ArbitraryEmrValue
    ) -> Option<ArbitraryEmrValue> {
        self.0.insert(key, value)
    }

    // update a batch of records at once
    pub fn update_batch(
        &mut self,
        records: Vec<(Stable<CompositeKey>, ArbitraryEmrValue)>
    ) -> Result<(), OutOfMemory> {
        for (key, value) in records {
            self.0.insert(key, value);
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

// // TODO: implement log for core registry
pub struct ActivityLog(Log<Stable<CompositeKey>, ArbitraryEmrValue, Memory>);

impl ActivityLog {
    pub fn add(&mut self, key: Stable<CompositeKey>, value: ArbitraryEmrValue) {
        self.0.insert(key, value);
    }

    pub fn update(
        &mut self,
        key: Stable<CompositeKey>,
        value: ArbitraryEmrValue
    ) -> Option<ArbitraryEmrValue> {
        self.0.insert(key, value)
    }

    // update a batch of records at once
    pub fn update_batch(
        &mut self,
        records: Log<Stable<CompositeKey>, ArbitraryEmrValue, Memory>
    ) -> Result<(), OutOfMemory> {
        for (key, value) in records {
            self.0.insert(key, value);
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
}