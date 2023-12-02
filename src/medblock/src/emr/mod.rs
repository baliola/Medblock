pub mod binding;

use candid::CandidType;
use ic_stable_memory::{
    collections::SHashMap,
    derive::{AsFixedSizeBytes, CandidAsDynSizeBytes, StableType},
    SBox,
};
use serde::Deserialize;

use crate::types::{EmrRecordsKey, Id, Timestamp};
/// version aware emr
#[derive(StableType, CandidType, Debug, CandidAsDynSizeBytes, Deserialize)]
#[non_exhaustive]
pub enum Emr {
    V001(V001),
}

#[derive(StableType, CandidAsDynSizeBytes, Debug, CandidType)]
pub struct Records(SHashMap<EmrRecordsKey, SBox<String>>);

#[derive(StableType, AsFixedSizeBytes, Debug, CandidType, Deserialize)]
pub struct V001 {
    emr_id: Id,
    created_at: Timestamp,
    updated_at: Timestamp,
    records: SBox<Records>,
}

mod deserialize {
    use super::*;

    pub struct RecordVisitor;

    impl<'de> serde::de::Visitor<'de> for RecordVisitor {
        type Value = Records;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut records = SHashMap::new();

            while let Some((key, value)) = map.next_entry::<EmrRecordsKey, String>()? {
                let value = SBox::new(value).map_err(serde::de::Error::custom)?;
                records.insert(key, value);
            }

            Ok(Records(records))
        }
    }

    impl<'de> serde::Deserialize<'de> for Records {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_map(RecordVisitor)
        }
    }
}
