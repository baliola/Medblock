pub mod binding;

use std::{collections::HashMap, marker::PhantomData};

use candid::CandidType;
use ic_stable_memory::{
    collections::SHashMap,
    derive::{AsFixedSizeBytes, CandidAsDynSizeBytes, StableType},
    SBox,
};
use serde::Deserialize;

use crate::{
    deref,
    types::{CanisterResponse, EmrRecordsKey, Id, Timestamp},
};

/// version aware emr
#[derive(StableType, AsFixedSizeBytes)]
#[non_exhaustive]
pub enum Emr {
    V001(V001),
}

#[derive(StableType, Debug, AsFixedSizeBytes)]
pub struct Records(SHashMap<EmrRecordsKey, SBox<String>>);
deref!(Records: SHashMap<EmrRecordsKey, SBox<String>>);

#[derive(AsFixedSizeBytes, StableType, Debug)]
pub struct V001 {
    emr_id: Id,
    created_at: Timestamp,
    updated_at: Timestamp,
    records: Records,
}

pub struct V001Presenter {
    emr_id: Id,
    created_at: Timestamp,
    updated_at: Timestamp,
    records: HashMap<EmrRecordsKey, String>,
}
