use candid::CandidType;
use canister_common::{
    common::{ ProviderId, Timestamp, UserId },
    deref,
    from,
    impl_max_size,
    impl_mem_bound,
    impl_range_bound,
    mmgr::MemoryManager,
    stable::{ Candid, Memory, Scale, Stable, StableSet, ToStable },
};
use ic_stable_structures::Log;
use parity_scale_codec::{ Decode, Encode };
use serde::Deserialize;

use crate::registry::NIK;

pub struct PatientLog {
    pub activity_log: ActivityLogEntry,
    pub log_map_index: LogMapIndex,
}

impl PatientLog {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        let activity_log = ActivityLogEntry::init(memory_manager);
        let log_map_index = LogMapIndex::init(memory_manager);
        Self { activity_log, log_map_index }
    }

    pub fn record(&mut self, activity_type: ActivityType, provider: ProviderId, user: UserId) {
        let activity = Activity::new(activity_type, provider, user.clone());
        let index = self.activity_log.add(&activity);
        self.log_map_index.add(user, index);
    }

    pub fn get_logs(&self, user: &UserId) -> Option<Vec<Stable<Activity, Candid>>> {
        self.log_map_index
            .get_batch(user)
            .map(|indexes| self.activity_log.get_batch(&indexes))
            .map(|activities|
                activities
                    .into_iter()
                    .filter_map(|activity| activity)
                    .collect()
            )
    }
}

pub struct ActivityIndexMemory;
pub struct ActivityEntryMemory;

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum ActivityType {
    Updated,
    Accessed,
    Revoked,
}

impl_max_size!(for ActivityType: ActivityType);
impl_mem_bound!(for ActivityType: bounded; fixed_size: true);

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Activity {
    pub activity_type: ActivityType,
    pub timestamp: Timestamp,
    pub provider_id: ProviderId,
    pub user_id: UserId,
}

impl Activity {
    pub fn new(activity_type: ActivityType, provider_id: ProviderId, user_id: UserId) -> Self {
        Self {
            activity_type,
            timestamp: Timestamp::new(),
            provider_id,
            user_id,
        }
    }
}

impl_max_size!(for Activity: 165);
impl_mem_bound!(for Activity: bounded; fixed_size: false);

#[cfg(test)]
mod activity_test {
    use std::str::FromStr;

    use canister_common::{ common::H256, id };

    use super::*;

    #[test]
    fn test_len_encoded() {
        use candid::{ Encode, Decode };

        let activity = Activity {
            activity_type: ActivityType::Updated,
            timestamp: Timestamp::new(),
            provider_id: id!("adad9d46-a795-4445-ac10-8f2d150064ba"),
            user_id: H256::from_str(
                "9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c"
            ).unwrap(),
        };

        let encoded = Encode!(&activity).unwrap();
        println!("encoded: {:?}", encoded.len());
        let decoded = Decode!(&encoded, Activity).unwrap();
    }
}

pub struct ActivityLogEntry(Log<Stable<Activity, Candid>, Memory, Memory>);

impl ActivityLogEntry {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        let index_mem = memory_manager.get_memory::<_, ActivityIndexMemory>(|mem| mem);
        let data_mem = memory_manager.get_memory::<_, ActivityEntryMemory>(|mem| mem);

        Self(Log::init(index_mem, data_mem).unwrap())
    }

    pub fn add(&mut self, activity: &Activity) -> u64 {
        self.0.append(activity.to_stable_ref()).expect("OOM ")
    }

    pub fn get(&self, index: u64) -> Option<Stable<Activity, Candid>> {
        self.0.get(index)
    }

    pub fn get_batch(&self, indexed: &[u64]) -> Vec<Option<Stable<Activity, Candid>>> {
        let mut batch = Vec::with_capacity(indexed.len());

        for index in indexed {
            batch.push(self.get(*index));
        }

        batch
    }
}

#[derive(
    Debug,
    Clone,
    CandidType,
    PartialEq,
    Eq,
    Deserialize,
    Default,
    Encode,
    Decode,
    PartialOrd,
    Ord
)]
#[repr(transparent)]
pub struct U64(u64);
deref!(mut U64: u64);
impl_max_size!(for U64: u64);
impl_mem_bound!(for U64: bounded; fixed_size: true);
impl_range_bound!(U64);

impl AsRef<u64> for U64 {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}
from!(U64:u64);

impl Into<u64> for U64 {
    fn into(self) -> u64 {
        self.0
    }
}

pub struct LogMapIndex(StableSet<Stable<NIK>, Stable<U64>>);

impl LogMapIndex {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self(StableSet::init::<Self>(memory_manager))
    }

    pub fn add(&mut self, nik: NIK, index: u64) {
        let index: Stable<U64, Scale> = U64::from(index).to_stable();
        self.0.insert(nik.to_stable(), index)
    }

    pub fn get_batch(&self, nik: &NIK) -> Option<Vec<u64>> {
        let idxs = self.0.get_set_associated_by_key(nik.to_stable_ref());

        if let Some(idxs) = idxs {
            Some(
                idxs
                    .into_iter()
                    .map(|idx| idx.into_inner().into())
                    .collect()
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use candid::Principal;
    use canister_common::{ id, memory_manager };

    use super::*;

    #[test]
    fn test_activity_log() {
        let memory_manager = memory_manager!();
        let mut activity_log = ActivityLogEntry::init(&memory_manager);

        let activity = Activity::new(
            ActivityType::Updated,
            Principal::anonymous().into(),
            UserId::from_str(
                "9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c"
            ).unwrap()
        );

        let index = activity_log.add(&activity);
        let retrieved = activity_log.get(index).unwrap();

        assert_eq!(activity, *retrieved);
    }

    #[test]
    fn test_batch() {
        let memory_manager = memory_manager!();
        let mut activity_log = ActivityLogEntry::init(&memory_manager);

        let activity = Activity::new(
            ActivityType::Updated,
            Principal::anonymous().into(),
            UserId::from_str(
                "9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c"
            ).unwrap()
        );

        let mut indexes = Vec::new();

        for i in 0..100 {
            indexes.push(activity_log.add(&activity));
        }

        let batch = activity_log.get_batch(&indexes);

        for retrieved_activity in batch {
            assert!(retrieved_activity.unwrap().eq(activity.to_stable_ref()));
        }
    }
}
