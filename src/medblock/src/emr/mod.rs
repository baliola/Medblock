pub mod binding;

use std::{collections::HashMap, marker::PhantomData};

use candid::CandidType;
use ic_stable_memory::{
    collections::SHashMap,
    derive::{AsFixedSizeBytes, CandidAsDynSizeBytes, StableType},
    SBox,
};
use serde::{Deserialize, Serialize};

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

impl CanisterResponse<Representer> for Emr {
    fn encode_json(&self) -> Representer {
        match self {
            Self::V001(v) => Representer::V001(V001Presenter::from_ref(&v)),
        }
    }
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

#[derive(serde::Serialize)]
#[non_exhaustive]
#[serde(tag = "version")]
pub enum Representer {
    V001(V001Presenter),
}

// TODO : optimize this later using lifetimes and such
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
