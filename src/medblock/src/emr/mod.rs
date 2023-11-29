use candid::{CandidType, Deserialize};
use ic_stable_memory::{
    collections::SHashMap,
    derive::{AsFixedSizeBytes, StableType},
    SBox,
};

use crate::deref;

#[non_exhaustive]
pub enum Emr {
    V001(),
}

pub type Timestamp = u64;
/// emr metadata key must not exceed 100 ascii characters
#[derive(
    CandidType,
    StableType,
    AsFixedSizeBytes,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug,
)]
pub struct EmrMetadataKey([u8; 100]);
deref!(EmrMetadataKey: [u8; 100]);

pub struct V001 {
    emr_id: uuid::Uuid,
    created_at: Timestamp,
    updated_at: Timestamp,
    records: SHashMap<EmrMetadataKey, SBox<String>>,
}
