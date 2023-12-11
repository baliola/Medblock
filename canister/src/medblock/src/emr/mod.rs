pub mod binding;
mod providers;

use std::collections::HashMap;

use candid::CandidType;
use ic_stable_memory::{
    collections::SHashMap,
    derive::{AsFixedSizeBytes, StableType},
    SBox,
};

use crate::{
    deref,
    types::{AsciiRecordsKey, CanisterResponse, Id, Timestamp},
};

use self::binding::{EmrBindingMap, OwnerMap};

#[derive(Default)]
pub struct Registry {
    owners: OwnerMap,
    owner_emrs: EmrBindingMap,
    core_emrs: EmrCollection,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub struct EmrCollection(ic_stable_memory::collections::SBTreeMap<Id, Emr>);
deref!(EmrCollection: ic_stable_memory::collections::SBTreeMap<Id,Emr>);
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

#[derive(StableType, Debug, AsFixedSizeBytes)]
pub struct EmrRecordsValue(SBox<String>);

impl EmrRecordsValue {
    fn value_from_ref(&self) -> serde_json::Value {
        self.0.as_str().into()
    }
}

#[derive(StableType, Debug, AsFixedSizeBytes)]
pub struct Records(SHashMap<AsciiRecordsKey, EmrRecordsValue>);
deref!(Records: SHashMap<AsciiRecordsKey, EmrRecordsValue>);

impl CandidType for Records {
    fn _ty() -> candid::types::Type {
        candid::types::Type::Text
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        let v: serde_json::Value = self
            .0
            .iter()
            .map(|(k, v)| (k.to_string(), v.value_from_ref()))
            .collect();

        String::idl_serialize(&v.to_string(), serializer)
    }
}

#[derive(AsFixedSizeBytes, StableType, Debug)]
pub struct V001 {
    emr_id: Id,
    created_at: Timestamp,
    updated_at: Timestamp,
    records: Records,
}