use candid::CandidType;
use ic_stable_memory::{collections::SHashSet, derive::{AsFixedSizeBytes, StableType}, StableType};


// #[derive(
//     CandidType, StableType, AsFixedSizeBytes, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug,
// )]
// pub struct Schema {
//     schema_id: ID,
//     created_at: Timestamp,
//     updated_at: Timestamp,
//     keys: SHashSet<EmrMetadataKey>,
// }
