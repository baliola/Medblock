use std::fmt::Debug;

use ic_stable_structures::BTreeMap;

use canister_common::{
    common::{
        canister_id,
        ArbitraryEmrValue,
        EmrBody,
        EmrHeaderWithBody,
        EmrId,
        Id,
        ProviderId,
        UserId,
    },
    log,
    metrics,
    mmgr::MemoryManager,
    stable::{ Memory, Stable, ToStable },
    statistics::traits::Metrics,
};

use crate::header::Header;

use self::key::*;

const MAGIC_RECORDS_KEY: RecordsKey = RecordsKey::static_key(*b"fcd25a10-7658-4384-97d8-9a161b2f");
const MAGIC_RECORDS_KEY_VALUE: &str = "magic";

use super::key::{
    ByEmr,
    ByRecordsKey,
    CompositeKey,
    CompositeKeyBuilder,
    Known,
    ProviderBatch,
    Threshold,
    Unknown,
    UserBatch,
    RecordsKey,
};

#[derive(thiserror::Error, Debug, candid::CandidType, serde::Deserialize)]
pub enum CoreRegistryError {
    #[error("The EMR does not exist")]
    NotExist,

    #[error("The EMR already exists")]
    AlreadyExists,
}

pub type RegistryResult<T> = Result<T, CoreRegistryError>;

pub mod key {
    use super::*;

    pub type MagicKey = CompositeKeyBuilder<
        ByRecordsKey,
        Known<UserId>,
        Known<ProviderId>,
        Known<EmrId>,
        Known<RecordsKey>
    >;

    pub type EmrKey = CompositeKeyBuilder<ByEmr, Known<UserId>, Known<ProviderId>, Known<EmrId>>;

    impl EmrKey {
        pub fn to_magic(self) -> MagicKey {
            MagicKey::new()
                .with_user(self.user_id.into_inner())
                .with_provider(self.provider_id.into_inner())
                .with_emr_id(self.emr_id.into_inner())
                .with_records_key(MAGIC_RECORDS_KEY)
        }
    }
    pub type AddEmrKey = CompositeKeyBuilder<
        ByRecordsKey,
        Known<UserId>,
        Known<ProviderId>,
        Known<EmrId>
    >;

    pub type UpdateKey = CompositeKeyBuilder<
        ByRecordsKey,
        Known<UserId>,
        Known<ProviderId>,
        Known<EmrId>,
        Known<RecordsKey>
    >;

    pub type PartialUpdateKey = CompositeKeyBuilder<
        ByRecordsKey,
        Known<UserId>,
        Known<ProviderId>,
        Known<EmrId>
    >;

    pub type UserBatchKey = CompositeKeyBuilder<UserBatch, Known<UserId>>;
    pub type ProviderBatchKey = CompositeKeyBuilder<
        ProviderBatch,
        Unknown<UserId>,
        Known<ProviderId>
    >;
}
pub struct CoreEmrRegistry(BTreeMap<Stable<CompositeKey>, ArbitraryEmrValue, Memory>);
metrics!(CoreEmrRegistry: TotalKeys);

impl Metrics<TotalKeys> for CoreEmrRegistry {
    fn metrics_name() -> &'static str {
        "total_emr_keys"
    }

    fn metrics_measurements() -> &'static str {
        "len"
    }

    fn update_measurements(&self) {
        // no-op
    }

    fn get_measurements(&self) -> String {
        self.0.len().to_string()
    }
}

impl CoreEmrRegistry {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        let tree = memory_manager.get_memory::<_, Self>(BTreeMap::init);
        Self(tree)
    }
}

impl Debug for CoreEmrRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_map();

        for (key, value) in self.0.iter() {
            let key = format!("{key} => ");
            let value = value.to_string();
            result.entry(&key, &value);
        }

        result.finish()
    }
}

impl CoreEmrRegistry {
    pub fn add(&mut self, key: AddEmrKey, emr: EmrBody) -> RegistryResult<Header> {
        let exists_key_check = EmrKey::new()
            .with_user(key.user_id.clone().into_inner())
            .with_provider(key.provider_id.clone().into_inner())
            .with_emr_id(key.emr_id.clone().into_inner());

        let magic_key = key.clone().with_records_key(MAGIC_RECORDS_KEY.clone()).build().to_stable();

        if self.is_emr_exists(exists_key_check).is_ok() {
            return Err(CoreRegistryError::AlreadyExists);
        }

        let header = Header::new(
            key.user_id.clone().into_inner(),
            key.provider_id.clone().into_inner(),
            key.emr_id.clone().into_inner(),
            canister_id()
        );

        // insert magic key
        self.0.insert(magic_key, MAGIC_RECORDS_KEY_VALUE.into());

        for fragment in emr.into_iter() {
            let (k, v) = (fragment.key, fragment.value);

            if k.eq(&MAGIC_RECORDS_KEY) {
                continue;
            }

            let emr_key = key.clone().with_records_key(k).build();
            self.0.insert(emr_key.into(), v);
        }

        Ok(header)
    }

    pub fn is_emr_exists(&self, key: EmrKey) -> RegistryResult<()> {
        let key = key.to_magic().build().to_stable();

        self.0.contains_key(&key).then_some(()).ok_or(CoreRegistryError::NotExist)
    }

    pub fn update(
        &mut self,
        key: UpdateKey,
        value: ArbitraryEmrValue
    ) -> Option<ArbitraryEmrValue> {
        let key = key.build().into();
        self.0.insert(key, value)
    }

    /// update given emr, will upsert if the the field does not exists.
    pub fn update_batch(
        &mut self,
        key: PartialUpdateKey,
        values: EmrBody
    ) -> RegistryResult<Header> {
        let check_key = EmrKey::new()
            .with_user(key.user_id.clone().into_inner())
            .with_provider(key.provider_id.clone().into_inner())
            .with_emr_id(key.emr_id.clone().into_inner());

        // ensure emr exists
        self.is_emr_exists(check_key)?;

        let header = Header::new(
            key.user_id.clone().into_inner(),
            key.provider_id.clone().into_inner(),
            key.emr_id.clone().into_inner(),
            canister_id()
        );

        for fragment in values {
            let (k, v) = (fragment.key, fragment.value);

            if k.eq(&MAGIC_RECORDS_KEY) {
                continue;
            }

            let records_key = key.clone().with_records_key(k);
            self.update(records_key, v);
        }

        Ok(header)
    }

    pub fn remove_record(&mut self, key: EmrKey) -> RegistryResult<()> {
        let key = key.build().to_stable();

        let keys_to_remove: Vec<_> = self.0
            .range(key.clone()..)
            .take_while(|(k, _)| k.emr_id() == key.emr_id())
            .map(|(k, _)| k.clone())
            .collect();

        if keys_to_remove.is_empty() {
            return Err(CoreRegistryError::NotExist);
        }

        for key in keys_to_remove {
            self.0.remove(&key);
        }

        Ok(())
    }

    /// Get the list of EMRs for a user, this will not filter by provider
    pub fn get_user_list_batch(&self, page: u64, limit: u64, key: UserBatchKey) -> Vec<Header> {
        let key = key.build().to_stable();
        self.get_list_batch::<UserId, UserBatch>(page, limit, &key)
    }

    /// Get the list of EMRs for a provider, this will not filter by user
    pub fn get_provider_batch(&self, page: u64, limit: u64, key: ProviderBatchKey) -> Vec<Header> {
        let key = key.build().to_stable();
        self.get_list_batch::<ProviderId, ProviderBatch>(page, limit, &key)
    }

    fn get_list_batch<U: Eq, T: Threshold<T = U>>(
        &self,
        page: u64,
        limit: u64,
        key: &Stable<CompositeKey>
    ) -> Vec<Header> {
        let start = page * limit;
        let end = start + limit;

        let mut last_id = Id::default();
        let mut index = 0;

        let iter = self.0.range(key..);

        let mut result = vec![];

        let threshold = T::threshold(key.as_inner());

        // Iterate over the range iterator
        for (k, _) in iter {
            // If the threshold of the current key does not match the provided threshold,
            // break the loop. This ensures that we only process entries with the same threshold.
            if T::threshold(k.as_inner()) != threshold {
                break;
            }

            // If the emr_id of the current key is the same as the last seen emr_id,
            // skip this iteration and continue with the next one. This ensures that we only process unique emr_ids.
            if k.emr_id() == &last_id {
                continue;
            }

            // If the current index has reached or exceeded the end of the page,
            // break the loop. This ensures that we stop processing entries once we've reached the end of the page.
            if index >= end {
                break;
            }

            // Update the last seen emr_id and increment the index.
            last_id = k.emr_id().clone();
            index += 1;

            // If the current index has reached or exceeded the start of the page,
            // add the emr_id of the current key to the result. This ensures that we only start adding emr_ids to the result
            // once we've reached the start of the page.
            if index >= start {
                result.push(k.into_inner().into());
            }
        }
        result
    }

    pub fn read_by_id(&self, key: EmrKey) -> RegistryResult<EmrHeaderWithBody> {
        let key = key.build().to_stable();

        let records = self.0
            .range(key.clone()..)
            .take_while(|(k, _)| k.emr_id() == key.emr_id())
            .filter(|(k, _)| k.record_key().ne(&MAGIC_RECORDS_KEY))
            .map(|(k, v)| (k.record_key().to_owned(), v.clone()))
            .collect::<Vec<_>>();

        if records.is_empty() {
            Err(CoreRegistryError::NotExist)
        } else {
            let header = Header::from(key.into_inner());
            let body = EmrBody::from(records);
            Ok(EmrHeaderWithBody::new(header.into(), body))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::key::UnknownUsage;

    use super::*;
    use canister_common::{ common::{ AsciiRecordsKey, EmrFragment, EmrHeader }, id };
    use ic_principal::Principal;

    #[test]
    fn test_core_emr_registry() {
        let memory_manager = MemoryManager::init();
        let mut registry = CoreEmrRegistry::init(&memory_manager);

        let user = id!("be06a4e7-bc46-4740-8397-ea00d9933cc1");
        let user = canister_common::test_utils::hash(user.as_bytes());
        let provider = id!("b0e6abc0-5b4f-49b8-b1cf-9f4a452ff22d");
        let emr_id = id!("6c5dd2ec-0fe0-40dc-ae33-234252be26ed");

        let key = CompositeKeyBuilder::<UnknownUsage>
            ::new()
            .records_key()
            .with_user(user.into())
            .with_provider(provider.clone())
            .with_emr_id(emr_id.clone());

        let records = vec![
            (AsciiRecordsKey::new("key1").unwrap(), ArbitraryEmrValue::from("value1")),
            (AsciiRecordsKey::new("key2").unwrap(), ArbitraryEmrValue::from("value2"))
        ];
        let emr = EmrBody::from(records);

        let header = registry.add(key.clone(), emr).unwrap();

        let key = CompositeKeyBuilder::<UnknownUsage>
            ::new()
            .emr()
            .with_user(header.user_id.clone())
            .with_provider(header.provider_id.clone())
            .with_emr_id(header.emr_id.clone());

        let result = registry.read_by_id(key.clone());
        assert!(result.is_ok());

        let key = CompositeKeyBuilder::<UnknownUsage>::new().user_batch().with_user(user.into());

        let result = registry.get_user_list_batch(0, 10, key);
        let header = EmrHeader {
            user_id: user.into(),
            emr_id: emr_id.clone(),
            provider_id: provider.clone(),
            registry_id: Principal::anonymous().into(),
        };
        assert_eq!(result, vec![Header::from(header.clone())]);

        let key = CompositeKeyBuilder::<UnknownUsage>
            ::new()
            .provider_batch()
            .with_provider(provider.clone());
        let result = registry.get_provider_batch(0, 10, key.clone());
        let header = EmrHeader::new(
            user.into(),
            emr_id.clone(),
            provider.clone(),
            Principal::anonymous()
        );
        assert_eq!(result, vec![Header(header)]);

        let key = CompositeKeyBuilder::<UnknownUsage>
            ::new()
            .emr()
            .with_user(user.into())
            .with_provider(provider.clone())
            .with_emr_id(emr_id.clone());

        registry.remove_record(key.clone());

        let result = registry.read_by_id(key.clone());
        assert!(result.is_err());
    }

    #[test]
    fn test_emr_exists() {
        let memory_manager = MemoryManager::init();
        let mut registry = CoreEmrRegistry::init(&memory_manager);

        let user = id!("be06a4e7-bc46-4740-8397-ea00d9933cc1");
        let user = canister_common::test_utils::hash(user.as_bytes());
        let provider = id!("b0e6abc0-5b4f-49b8-b1cf-9f4a452ff22d");
        let emr_id = id!("6c5dd2ec-0fe0-40dc-ae33-234252be26ed");

        let key = CompositeKeyBuilder::<UnknownUsage>
            ::new()
            .records_key()
            .with_user(user.into())
            .with_provider(provider.clone())
            .with_emr_id(emr_id.clone());

        let records = vec![
            (AsciiRecordsKey::new("key1").unwrap(), ArbitraryEmrValue::from("value1")),
            (AsciiRecordsKey::new("key4").unwrap(), ArbitraryEmrValue::from("value1"))
        ];
        let emr = EmrBody::from(records);

        let header = registry.add(key.clone(), emr).unwrap();

        let key = CompositeKeyBuilder::<UnknownUsage>
            ::new()
            .emr()
            .with_user(header.user_id.clone())
            .with_provider(header.provider_id.clone())
            .with_emr_id(header.emr_id.clone());

        let result = registry.is_emr_exists(key.clone());

        assert!(result.is_ok());

        registry.remove_record(key.clone());

        let result = registry.is_emr_exists(key.clone());

        assert!(result.is_err());
    }

    #[test]
    fn test_2_emr_exists() {
        let memory_manager = MemoryManager::init();
        let mut registry = CoreEmrRegistry::init(&memory_manager);

        let user = id!("be06a4e7-bc46-4740-8397-ea00d9933cc1");
        let user = canister_common::test_utils::hash(user.as_bytes());
        let provider = id!("b0e6abc0-5b4f-49b8-b1cf-9f4a452ff22d");
        let emr_id = id!("6c5dd2ec-0fe0-40dc-ae33-234252be26ed");

        let key = CompositeKeyBuilder::<UnknownUsage>
            ::new()
            .records_key()
            .with_user(user.into())
            .with_provider(provider.clone())
            .with_emr_id(emr_id.clone());

        let records = vec![
            (AsciiRecordsKey::new("key1").unwrap(), ArbitraryEmrValue::from("value1")),
            (AsciiRecordsKey::new("key4").unwrap(), ArbitraryEmrValue::from("value1"))
        ];
        let emr = EmrBody::from(records);

        let header = registry.add(key.clone(), emr).unwrap();

        let key = CompositeKeyBuilder::<UnknownUsage>
            ::new()
            .emr()
            .with_user(header.user_id.clone())
            .with_provider(header.provider_id.clone())
            .with_emr_id(header.emr_id.clone());

        let _result = registry.is_emr_exists(key.clone());

        let inexistent_key = id!("61e092c4-22fe-4f51-8450-fea3d0d9eb0a");
        let key = CompositeKeyBuilder::<UnknownUsage>
            ::new()
            .emr()
            .with_user(header.user_id.clone())
            .with_provider(header.provider_id.clone())
            .with_emr_id(inexistent_key.clone());

        let result = registry.is_emr_exists(key.clone());

        println!("inexistent key: {}", key.build());
        println!("{:#?}", registry);

        assert!(result.is_err(), "emr should not exists");
    }

    #[test]
    fn test_upsert_emr() {
        let memory_manager = MemoryManager::init();
        let mut registry = CoreEmrRegistry::init(&memory_manager);

        let user = id!("be06a4e7-bc46-4740-8397-ea00d9933cc1");
        let user = canister_common::test_utils::hash(user.as_bytes());
        let provider = id!("b0e6abc0-5b4f-49b8-b1cf-9f4a452ff22d");
        let emr_id = id!("6c5dd2ec-0fe0-40dc-ae33-234252be26ed");

        let key = CompositeKeyBuilder::<UnknownUsage>
            ::new()
            .records_key()
            .with_user(user.into())
            .with_provider(provider.clone())
            .with_emr_id(emr_id.clone());

        let records = vec![
            (AsciiRecordsKey::new("key1").unwrap(), ArbitraryEmrValue::from("value1")),
            (AsciiRecordsKey::new("key4").unwrap(), ArbitraryEmrValue::from("value1"))
        ];
        let emr = EmrBody::from(records);

        let header = registry.add(key.clone(), emr.clone()).unwrap();
        assert!(registry.add(key.clone(), emr.clone()).is_err());

        let _key = CompositeKeyBuilder::<UnknownUsage>
            ::new()
            .emr()
            .with_user(header.user_id.clone())
            .with_provider(header.provider_id.clone())
            .with_emr_id(header.emr_id.clone());

        let new_fields = vec![
            EmrFragment::new("new_field_1".try_into().unwrap(), "new value 1".to_string()),
            EmrFragment::new("new_field_2".try_into().unwrap(), "new value 2".to_string())
        ];

        let total_fields = [emr.clone().into_inner(), new_fields.clone()].concat();

        let header = registry
            .update_batch(header.to_partial_update_key(), new_fields.clone().into())
            .unwrap();

        println!("{:#?}", registry);
        let emrs = registry.read_by_id(header.to_emr_key()).unwrap().into_inner_body();

        assert!(emrs.clone().into_inner().len() == total_fields.len());

        for fragment in emrs {
            assert!(total_fields.contains(&fragment));
        }
    }
}
