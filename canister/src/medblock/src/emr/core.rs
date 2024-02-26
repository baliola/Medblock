use std::fmt::Debug;


use ic_stable_structures::{ BTreeMap };


use crate::{
    internal_types::{ AsciiRecordsKey, Id },
    mem::shared::{ Memory, Stable, ToStable },
};

use super::key::{
    ArbitraryEmrValue,
    ByEmr,
    ByRecordsKey,
    CompositeKey,
    CompositeKeyBuilder,
    EmrId,
    Known,
    ProviderBatch,
    ProviderId,
    RecordsKey,
    Threshold,
    Unknown,
    UserBatch,
    UserId,
};

pub struct CoreEmrRegistry(BTreeMap<Stable<CompositeKey>, ArbitraryEmrValue, Memory>);

impl CoreEmrRegistry {
    pub fn new(memory_manager: &crate::mem::MemoryManager) -> Self {
        let tree = memory_manager.get_memory(BTreeMap::new);
        Self(tree)
    }
}

impl Debug for CoreEmrRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("CoreRegistry");

        for (key, value) in self.0.iter() {
            result.field(&format!("{:?}", key), &format!("{:?}", value));
        }

        result.finish()
    }
}

impl CoreEmrRegistry {
    pub fn add(
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

    /// Get the list of EMRs for a user, this will not filter by provider
    pub fn get_user_list_batch(
        &self,
        page: u64,
        limit: u64,
        key: CompositeKeyBuilder<UserBatch, Known<UserId>>
    ) -> Vec<EmrId> {
        let key = key.build().to_stable();
        self.get_list_batch::<UserBatch>(page, limit, &key)
    }

    /// Get the list of EMRs for a provider, this will not filter by user
    pub fn get_provider_batch(
        &self,
        page: u64,
        limit: u64,
        key: CompositeKeyBuilder<ProviderBatch, Unknown<UserId>, Known<ProviderId>>
    ) -> Vec<EmrId> {
        let key = key.build().to_stable();
        self.get_list_batch::<ProviderBatch>(page, limit, &key)
    }

    fn get_list_batch<T: Threshold<T = Id>>(
        &self,
        page: u64,
        limit: u64,
        key: &Stable<CompositeKey>
    ) -> Vec<EmrId> {
        let start = page * limit;
        let end = start + limit;

        let mut last_id = Id::default();
        let mut index = 0;

        let iter = self.0.range(key..);

        let mut result = vec![];

        let threshold = T::threshold(key.as_inner());

        for (k, _) in iter {
            if T::threshold(k.as_inner()) != threshold {
                break;
            }

            if k.emr_id() == &last_id {
                continue;
            }

            if index >= start && index < end {
                result.push(k.emr_id().clone());
            }

            last_id = k.emr_id().clone();
            index += 1;
        }

        result
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

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    

    use super::*;
    use crate::{ fake_memory_manager, id };

    #[test]
    fn test_core_emr_registry() {
        let memory_manager = fake_memory_manager!();
        let mut registry = CoreEmrRegistry::new(&memory_manager);

        let user = id!("be06a4e7-bc46-4740-8397-ea00d9933cc1");
        let provider = id!("b0e6abc0-5b4f-49b8-b1cf-9f4a452ff22d");
        let emr_id = id!("6c5dd2ec-0fe0-40dc-ae33-234252be26ed");

        let key = CompositeKeyBuilder::new()
            .records_key()
            .with_user(user.clone())
            .with_provider(provider.clone())
            .with_emr_id(emr_id.clone());

        let records = vec![
            (AsciiRecordsKey::new("key1").unwrap(), ArbitraryEmrValue::from("value1")),
            (AsciiRecordsKey::new("key2").unwrap(), ArbitraryEmrValue::from("value2"))
        ];
        let emr = RawEmr::from(records);

        registry.add(key.clone(), emr);

        let key = CompositeKeyBuilder::new()
            .emr()
            .with_user(user.clone())
            .with_provider(provider.clone())
            .with_emr_id(emr_id.clone());

        let result = registry.read_by_id(key.clone());
        assert!(result.is_some());

        let key = CompositeKeyBuilder::new().user_batch().with_user(user.clone());

        let result = registry.get_user_list_batch(0, 10, key);
        assert_eq!(result, vec![emr_id.clone()]);

        let key = CompositeKeyBuilder::new().provider_batch().with_provider(provider.clone());
        let result = registry.get_provider_batch(0, 10, key.clone());
        assert_eq!(result, vec![emr_id.clone()]);

        let key = CompositeKeyBuilder::new()
            .emr()
            .with_user(user.clone())
            .with_provider(provider.clone())
            .with_emr_id(emr_id.clone());
        registry.remove_record(key.clone());

        let result = registry.read_by_id(key.clone());
        assert!(result.is_none());
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
