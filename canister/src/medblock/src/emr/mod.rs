pub mod patient;
pub mod providers;

use std::collections::HashMap;

use candid::{CandidType, Principal};
use ic_stable_memory::{
    collections::SHashMap,
    derive::{AsFixedSizeBytes, StableType},
    primitive::s_ref::SRef,
    AsDynSizeBytes, AsFixedSizeBytes, SBox, StableType,
};

use crate::{
    deref, measure_alloc,
    types::{AsciiRecordsKey, Id, Timestamp},
};

use self::{
    patient::{EmrBindingMap, OwnerMap},
    providers::Providers,
};

#[derive(Default)]
pub struct EmrRegistry {
    owners: OwnerMap,
    owner_emrs: EmrBindingMap,
    core_emrs: EmrCollection,
}

impl EmrRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_owner_of_emr(&self, owner: &Principal, emr_id: &Id) -> bool {
        let Some(nik) = self.owners.get_nik(owner) else {
            return false;
        };

        self.owner_emrs.is_owner_of(&nik, emr_id)
    }

    pub fn is_valid_patient(&self, owner: &patient::Owner) -> bool {
        self.owners.is_valid_owner(owner)
    }

    pub fn get_emr(&self, emr_id: &Id) -> Option<SRef<'_, Emr>> {
        self.core_emrs.get_emr(emr_id)
    }
}

type EmrId = Id;
#[derive(Default)]
pub struct EmrCollection(ic_stable_memory::collections::SBTreeMap<EmrId, Emr>);

impl EmrCollection {
    pub fn get_emr(&self, emr_id: &EmrId) -> Option<SRef<'_, Emr>> {
        self.0.get(emr_id)
    }
}
deref!(mut EmrCollection: ic_stable_memory::collections::SBTreeMap<EmrId,Emr>);
measure_alloc!("emr_collection_with_10_thousands_emr_10_records": {
    let mut emr_collection = EmrCollection::default();

    for i in 0..10_000 {
        let mut emr = V001::default();

        for i in 0..10 {
            emr.records.insert(
                AsciiRecordsKey::new(format!("test{}", i)).unwrap(),
                EmrRecordsValue::new(format!("test{}", i)).unwrap(),
            );
        }

        emr_collection.insert(
            Id::new(),
            Emr::V001(V001::default()),
        );
    }



    emr_collection
});
/// version aware emr
#[derive(StableType, AsFixedSizeBytes, Debug, CandidType)]
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

/// Error when allocating something to stable memory due to stable memory exhaustion
#[derive(Debug)]
pub struct OutOfMemory;

impl<T> From<T> for OutOfMemory
where
    T: StableType,
{
    fn from(_: T) -> Self {
        Self
    }
}

impl std::fmt::Display for OutOfMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "stable memory exhausted")
    }
}

/// wrapper types for emr records, essentially just a [SBox] around [String].
/// required because we need to implement function to serialize this to [serde_json::Value] for [Records] type
#[derive(StableType, Debug, AsFixedSizeBytes)]
pub struct EmrRecordsValue(SBox<String>);
deref!(EmrRecordsValue: SBox<String>);

impl EmrRecordsValue {
    pub fn value_from_ref(&self) -> serde_json::Value {
        self.0.as_str().into()
    }

    /// create new [EmrRecordsValue] from [String], returns [OutOfMemory] if stable memory is exhausted
    pub fn new(value: impl Into<String>) -> Result<EmrRecordsValue, OutOfMemory> {
        let value = value.into();
        let value = SBox::new(value)?;

        Ok(Self(value))
    }
}

#[derive(StableType, Debug, AsFixedSizeBytes, Default)]
pub struct Records(SHashMap<AsciiRecordsKey, EmrRecordsValue>);
deref!(mut Records: SHashMap<AsciiRecordsKey, EmrRecordsValue>);

measure_alloc!("records": {
       let mut records = Records::default();

       records.insert(
           AsciiRecordsKey::new("test".to_string()).unwrap(),
           EmrRecordsValue::new("test").unwrap(),
       );

       records
});

impl Clone for Records {
    fn clone(&self) -> Self {
        todo!();

        // TODO : fix this, we're using stable memory as our main memory, but there is some case
        // such as cloning a emr copy that would result in stable memory allocation while we want to use the heap for that  as we didn't store any data after
        // the response has been serialized and sent. it's like using hard disk as a ram, but you want the volatility of ram.

    }
}

impl Records {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_value(&self) -> serde_json::Value {
        self.0
            .iter()
            .map(|(k, v)| (k.to_string(), v.value_from_ref()))
            .collect()
    }
}

impl CandidType for Records {
    fn _ty() -> candid::types::Type {
        candid::types::Type::Text
    }

    // TODO:  this copies ALOT of data
    // because we iterate and serializing the data to serde json Value type while copying
    // and then after that we copy again to serialize the Value type to String so that it can be properly serialized as candid type
    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        let v = self.to_value();
        String::idl_serialize(&v.to_string(), serializer)
    }
}

#[derive(AsFixedSizeBytes, StableType, Debug, Default, CandidType, Clone)]
pub struct V001 {
    emr_id: Id,
    created_at: Timestamp,
    updated_at: Timestamp,
    records: Records,
}
measure_alloc!("emr_with_10_records":{
    let mut emr = V001::default();

    for i in 0..10 {
        emr.records.insert(
            AsciiRecordsKey::new(format!("test{}", i)).unwrap(),
            EmrRecordsValue::new(format!("test{}", i)).unwrap(),
        );
    }

    emr
});

impl V001 {
    pub fn new() -> Self {
        Self::default()
    }
}
