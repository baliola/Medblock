use crate::{ deref, types::{ Id, Timestamp } };
use candid::{ CandidType, Principal };
use ic_stable_memory::{
    collections::SLog as Log,
    derive::{ AsFixedSizeBytes, CandidAsDynSizeBytes, StableType },
    SBox,
};
use serde::Deserialize;
// TODO : rearrange this to session logs
#[derive(StableType, CandidType, Debug, Deserialize, AsFixedSizeBytes)]
pub struct RecordsV001 {
    emr_id: Id,
    provider: Principal,
}

impl RecordsV001 {
    pub fn new(emr_id: Id, provider: Principal) -> Self {
        Self { emr_id, provider }
    }
}

#[derive(StableType, CandidType, Debug, CandidAsDynSizeBytes, Deserialize)]
#[non_exhaustive]
pub enum EntryRecords {
    V001(RecordsV001),
}

#[derive(CandidType, StableType, Debug, AsFixedSizeBytes)]
pub struct Entry {
    entry_id: Id,
    timestamp: Timestamp,
    records: SBox<EntryRecords>,
}

impl Entry {
    pub fn new(entry: EntryRecords, id: Id) -> Result<Self, EntryRecords> {
        Ok(Self {
            entry_id: id,
            timestamp: Timestamp::new(),
            records: SBox::new(entry)?,
        })
    }
}

pub struct EntryLog(Log<Entry>);
deref!(EntryLog: Log<Entry>);

impl Default for EntryLog {
    fn default() -> Self {
        Self(Log::new())
    }
}
