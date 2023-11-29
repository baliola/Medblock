mod schema;

use ic_stable_memory::{
    collections::SHashMap,
    derive::{AsFixedSizeBytes, StableType},
    SBox,
};
use uuid::Uuid;

use crate::{deref, types::{Id, Timestamp, EmrRecordsKey}};


/// version aware emr
#[non_exhaustive]
pub enum Emr {
    V001(V001),
}


#[derive(StableType, AsFixedSizeBytes, Debug)]
pub struct V001 {
    emr_id: Id,
    created_at: Timestamp,
    updated_at: Timestamp,
    records: SHashMap<EmrRecordsKey, SBox<String>>,
}
