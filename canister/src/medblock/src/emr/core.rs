use std::fmt::Debug;

use ic_stable_memory::OutOfMemory;
use ic_stable_structures::{ storable::Bound, BTreeMap, Log };
use parity_scale_codec::{ Decode, Encode };

use crate::{
    impl_max_size,
    internal_types::{ AsciiRecordsKey, Id },
    mem::shared::{ MemBoundMarker, Memory, Stable, ToStable },
};

use super::key::{
    ArbitraryEmrValue,
    ByEmr,
    ByRecordsKey,
    CompositeKey,
    CompositeKeyBuilder,
    EmrId,
    Known,
    ProviderId,
    RecordsKey,
    UserId,
};

pub struct CoreRegistry(BTreeMap<Stable<CompositeKey>, ArbitraryEmrValue, Memory>);

impl CoreRegistry {
    pub fn new(memory_manager: &crate::mem::MemoryManager) -> Self {
        let tree = memory_manager.get_memory(|mem| BTreeMap::new(mem));
        Self(tree)
    }
}

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
    pub fn add_batch(
        &mut self,
        key: CompositeKeyBuilder<ByRecordsKey, Known<UserId>, Known<ProviderId>, Known<EmrId>>,
        emr: RawEmr
    ) {
        for (k, v) in emr.into_iter() {
            let emr_key = key.clone().with_records_key(k).build();
            self.0.insert(emr_key.into(), v);
        }
    }

    pub fn update(
        &mut self,
        key: CompositeKeyBuilder<
            ByRecordsKey,
            Known<UserId>,
            Known<ProviderId>,
            Known<EmrId>,
            Known<RecordsKey>
        >,
        value: ArbitraryEmrValue
    ) -> Option<ArbitraryEmrValue> {
        let key = key.build().into();
        self.0.insert(key, value)
    }
    pub fn remove_record(
        &mut self,
        key: CompositeKeyBuilder<ByEmr, Known<UserId>, Known<ProviderId>, Known<EmrId>>
    ) {
        let key = key.build().to_stable();

        let keys_to_remove: Vec<_> = self.0
            .range(key.clone()..)
            .take_while(|(k, _)| k.emr_id() == key.emr_id())
            .map(|(k, _)| k.clone())
            .collect();

        for key in keys_to_remove {
            self.0.remove(&key);
        }
    }

    pub fn get_list_batch(&self, page: u64, limit: u64, key: &Stable<CompositeKey>) -> Vec<EmrId> {
        let start = page * limit;
        let end = start + limit;

        self.0
            .range(key..)
            .skip(start as usize)
            .take(limit as usize)
            .map(|(k, _)| k.emr_id().to_owned())
            .collect::<Vec<_>>()
    }

    pub fn read_by_id(
        &self,
        key: CompositeKeyBuilder<ByEmr, Known<UserId>, Known<ProviderId>, Known<EmrId>>
    ) -> Option<RawEmr> {
        let key = key.build().to_stable();

        let records = self.0
            .range(key.clone()..)
            .take_while(|(k, _)| k.emr_id() == key.emr_id())
            .map(|(k, v)| (k.record_key().to_owned(), v.clone()))
            .collect::<Vec<_>>();

        if records.is_empty() {
            None
        } else {
            Some(RawEmr::from(records))
        }
    }
}

pub struct RawEmr(Vec<(AsciiRecordsKey, ArbitraryEmrValue)>);

impl From<Vec<(AsciiRecordsKey, ArbitraryEmrValue)>> for RawEmr {
    fn from(records: Vec<(AsciiRecordsKey, ArbitraryEmrValue)>) -> Self {
        Self(records)
    }
}

impl IntoIterator for RawEmr {
    type Item = (AsciiRecordsKey, ArbitraryEmrValue);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
