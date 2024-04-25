use candid::CandidType;
use canister_common::{
    common::{ ProviderId, Timestamp, UserId },
    impl_max_size,
    impl_mem_bound,
    mmgr::MemoryManager,
    stable::{ Candid, Memory, Stable, ToStable },
};
use ic_stable_structures::Log;
use serde::Deserialize;

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
        Self { activity_type, timestamp: Timestamp::new(), provider_id, user_id }
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

pub struct ActivityLog(Log<Stable<Activity, Candid>, Memory, Memory>);

impl ActivityLog {
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

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use canister_common::{ id, memory_manager };

    use super::*;

    #[test]
    fn test_activity_log() {
        let memory_manager = memory_manager!();
        let mut activity_log = ActivityLog::init(&memory_manager);

        let activity = Activity::new(
            ActivityType::Updated,
            id!("adad9d46-a795-4445-ac10-8f2d150064ba"),
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
        let mut activity_log = ActivityLog::init(&memory_manager);

        let activity = Activity::new(
            ActivityType::Updated,
            id!("adad9d46-a795-4445-ac10-8f2d150064ba"),
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
