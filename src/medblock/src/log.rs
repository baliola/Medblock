use candid::{CandidType, Principal};
use ic_stable_memory::{
    collections::SLog,
    StableType,
    AsFixedSizeBytes,
    derive::{ AsFixedSizeBytes, StableType },
};
use serde::Deserialize;

use crate::types::Id;

pub struct Log<T>(SLog<T>) where T: StableType + AsFixedSizeBytes;

#[derive(CandidType, StableType, AsFixedSizeBytes, Debug, Deserialize)]
pub struct Entry {
    timestamp: u64,
    emr_id: Id,
    provider: Principal,
}

pub type EntryLog = Log<Entry>;
