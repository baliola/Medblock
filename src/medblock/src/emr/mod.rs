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

use self::binding::{EmrBindingMap, OwnerMap};

pub struct Registry {
    owners: OwnerMap,
    owner_emrs: EmrBindingMap,
    core_emrs: EmrCollection,
}

pub struct EmrCollection(ic_stable_memory::collections::SBTreeSet<Emr>);
/// version aware emr
#[derive(StableType, AsFixedSizeBytes, Debug)]
#[non_exhaustive]
pub enum Emr {
    V001(V001),
}

impl std::cmp::Eq for Emr {}

impl std::cmp::PartialEq for Emr {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Emr {
    pub fn id(&self) -> &Id {
        match self {
            Self::V001(v) => &v.emr_id,
        }
    }
}

impl std::cmp::Ord for Emr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id().cmp(other.id())
    }
}

impl std::cmp::PartialOrd for Emr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id().partial_cmp(other.id())
    }
}

impl CanisterResponse<SerializeableEmrResponse> for Emr {
    fn encode_json(&self) -> SerializeableEmrResponse {
        match self {
            Self::V001(v) => {
                SerializeableEmrResponse::V001(V001SerializeableEmrResponse::from_ref(&v))
            }
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

#[derive(serde::Serialize, Debug)]
#[non_exhaustive]
#[serde(tag = "version")]
pub enum SerializeableEmrResponse {
    V001(V001SerializeableEmrResponse),
}

// TODO : optimize this later using lifetimes and such
#[derive(serde::Serialize, Debug)]
pub struct V001SerializeableEmrResponse {
    emr_id: Id,
    created_at: Timestamp,
    updated_at: Timestamp,
    records: HashMap<EmrRecordsKey, String>,
}

impl V001SerializeableEmrResponse {
    fn from_ref(value: &V001) -> Self {
        V001SerializeableEmrResponse {
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

mod tests {
    use super::*;

    #[test]
    fn instruction_count() {
        ic_stable_memory::stable_memory_init();

        let mut records = Records(SHashMap::new());

        records
            .0
            .insert(
                EmrRecordsKey::new("key").unwrap(),
                SBox::new(String::from("value")).unwrap(),
            )
            .unwrap();
        records
            .0
            .insert(
                EmrRecordsKey::new("key2").unwrap(),
                SBox::new(String::from("value2")).unwrap(),
            )
            .unwrap();
        records
            .0
            .insert(
                EmrRecordsKey::new("key3").unwrap(),
                SBox::new(String::from("value3")).unwrap(),
            )
            .unwrap();

        let emr_id = Id::new();
        let dummy_timestamp = Timestamp(0);
        let mut emr = Emr::V001(V001 {
            emr_id: emr_id.clone(),
            created_at: dummy_timestamp,
            updated_at: dummy_timestamp,
            records,
        });

        let encoded = emr.encode_json();
        let encoded = serde_json::to_value(&encoded).unwrap();

        assert_eq!(
            encoded,
            serde_json::json!({
                "version": "V001",
                "emr_id": emr_id,
                "created_at": dummy_timestamp,
                "updated_at": dummy_timestamp,
                "records": {
                    "key": "value",
                    "key2": "value2",
                    "key3": "value3",
                }
            })
        )
    }
}
