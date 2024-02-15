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
    pub fn add(&mut self, key: CompositeKey, value: ArbitraryEmrValue) {
        self.0.insert(key.into(), value);
    }

    pub fn update(
        &mut self,
        key: Stable<CompositeKey>,
        value: ArbitraryEmrValue
    ) -> Option<ArbitraryEmrValue> {
        self.0.insert(key, value)
    }

    // update a batch of records at once
    pub fn update_batch(&mut self, records: Vec<(Stable<CompositeKey>, ArbitraryEmrValue)>) {
        for (key, value) in records {
            self.0.insert(key, value);
        }
    }

    pub fn remove_record(&mut self, key: Stable<CompositeKey>) -> bool {
        self.0.remove(&key).is_some()
    }

    pub fn get_list_provider_batch(&self, page: u64, limit: u64, key: &Stable<CompositeKey>) -> Vec<EmrId> {
        let start = page * limit;
        let end = start + limit;

        self.0
            .range((key)..)
            .skip(start as usize)
            .take(limit as usize)
            .map(|(k, _)| k.emr_id().to_owned())
            .collect::<Vec<_>>()
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
// pub struct ActivityLog(
//     // store the log in a vector
//     logs: Vec<Log<String>>,
// );

// implement from struc to log
// impl ActivityLog {
//     pub fn new() -> Self {
//         Self { logs: Vec::new() }
//     }

//     pub fn add(&mut self, log: Log<String>) {
//         self.logs.push(log);
//     }

//     pub fn get_logs(&self) -> Vec<Log<String>> {
//         self.logs.clone()
//     }
// }
