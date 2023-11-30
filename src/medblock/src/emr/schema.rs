use candid::Principal;
use ic_stable_memory::{
    collections::SHashSet,
    derive::{ AsFixedSizeBytes, StableType },
    StableType,
};

use crate::types::{ Id, Timestamp, EmrRecordsKey };

#[derive(StableType, AsFixedSizeBytes, Debug)]
pub struct EmrSchema {
    owned_by: Principal,
    schema_id: Id,
    created_at: Timestamp,
    updated_at: Timestamp,
    keys: SHashSet<EmrRecordsKey>,
}
