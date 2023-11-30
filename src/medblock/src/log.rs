use crate::{types::{Id, Timestamp}, deref};
use candid::{CandidType, Principal};
use ic_stable_memory::{
    collections::SLog as Log,
    derive::{AsFixedSizeBytes, StableType},
    AsFixedSizeBytes, StableType,
};
use serde::Deserialize;

#[derive(CandidType, StableType, AsFixedSizeBytes, Debug, Deserialize)]
pub struct Entry {
    timestamp: Timestamp,
    emr_id: Id,
    provider: Principal,
}

impl Entry {
    pub fn new(emr_id: Id, provider: Principal) -> Self {
        Self {
            timestamp: Timestamp::new(),
            emr_id,
            provider,
        }
    }
}

pub struct EntryLog(Log<Entry>);
deref!(EntryLog: Log<Entry>);

impl Default for EntryLog {
    fn default() -> Self {
        Self(Log::new())
    }
}
