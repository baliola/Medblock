mod schema;

use candid::{CandidType, Deserialize};
use ic_stable_memory::{
    collections::SHashMap,
    derive::{AsFixedSizeBytes, StableType},
    SBox,
};
use uuid::Uuid;

use crate::{deref, types::{ID, Timestamp, EmrMetadataKey}};

#[non_exhaustive]
pub enum Emr {
    V001(V001),
}

#[derive(StableType, AsFixedSizeBytes, Debug)]
pub struct V001 {
    emr_id: ID,
    created_at: Timestamp,
    updated_at: Timestamp,
    records: SHashMap<EmrMetadataKey, SBox<String>>,
}
