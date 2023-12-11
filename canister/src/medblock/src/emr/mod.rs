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
            .map(|(k, v)| (k.to_ascii_str().to_string(), v.value_from_ref()))
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

// #[derive(serde::Serialize, Debug)]
// #[non_exhaustive]
// #[serde(tag = "version")]
// pub enum SerializeableEmrResponse {
//     V001(V001SerializeableEmrResponse),
// }

// // TODO : optimize this later using lifetimes and such
// #[derive(serde::Serialize, Debug)]
// pub struct V001SerializeableEmrResponse {
//     emr_id: Id,
//     created_at: Timestamp,
//     updated_at: Timestamp,
//     records: HashMap<AsciiRecordsKey, String>,
// }

// impl V001SerializeableEmrResponse {
//     fn from_ref(value: &V001) -> Self {
//         V001SerializeableEmrResponse {
//             emr_id: value.emr_id.clone(),
//             created_at: value.created_at,
//             updated_at: value.updated_at,
//             records: value
//                 .records
//                 .iter()
//                 .map(|(k, v)| (k.to_owned(), v.to_owned()))
//                 .collect(),
//         }
//     }
// }

// mod tests {
//     #[allow(unused_imports)]
//     use super::*;

//     #[test]
//     fn instruction_count() {
//         ic_stable_memory::stable_memory_init();

//         let mut records = Records(SHashMap::new());

//         records
//             .0
//             .insert(
//                 AsciiRecordsKey::new("key").unwrap(),
//                 SBox::new(String::from("value")).unwrap(),
//             )
//             .unwrap();
//         records
//             .0
//             .insert(
//                 AsciiRecordsKey::new("key2").unwrap(),
//                 SBox::new(String::from("value2")).unwrap(),
//             )
//             .unwrap();
//         records
//             .0
//             .insert(
//                 AsciiRecordsKey::new("key3").unwrap(),
//                 SBox::new(String::from("value3")).unwrap(),
//             )
//             .unwrap();

//         let emr_id = Id::new();
//         let dummy_timestamp = Timestamp(0);
//         let emr = Emr::V001(V001 {
//             emr_id: emr_id.clone(),
//             created_at: dummy_timestamp,
//             updated_at: dummy_timestamp,
//             records,
//         });

//         let encoded = emr.encode_json();
//         let encoded = serde_json::to_value(encoded).unwrap();

//         assert_eq!(
//             encoded,
//             serde_json::json!({
//                 "version": "V001",
//                 "emr_id": emr_id,
//                 "created_at": dummy_timestamp,
//                 "updated_at": dummy_timestamp,
//                 "records": {
//                     "key": "value",
//                     "key2": "value2",
//                     "key3": "value3",
//                 }
//             })
//         )
//     }
// }
