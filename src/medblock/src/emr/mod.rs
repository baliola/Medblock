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
    types::{CanisterResponse, EmrRecordsKey, Id, Timestamp, ToSerialize},
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

impl ToSerialize<V001Presenter> for V001 {
    fn to_serialize(&self) -> V001Presenter {
        V001Presenter::from_ref(self)
    }
}

#[derive(serde::Serialize, Debug)]
pub struct V001Presenter {
    emr_id: Id,
    created_at: Timestamp,
    updated_at: Timestamp,
    records: HashMap<EmrRecordsKey, String>,
}

impl V001Presenter {
    fn from_ref(value: &V001) -> Self {
        V001Presenter {
            emr_id: value.emr_id.clone(),
            created_at: value.created_at.clone(),
            updated_at: value.updated_at.clone(),
            records: value
                .records
                .iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
        }
    }
}
