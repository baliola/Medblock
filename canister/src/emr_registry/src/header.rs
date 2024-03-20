use candid::CandidType;
use canister_common::{ common::{ EmrId, ProviderId, UserId }, from };
use serde::Deserialize;

use crate::key::CompositeKey;

#[derive(Debug, Deserialize, CandidType, PartialEq, Eq)]
pub struct EmrHeader {
    pub user_id: UserId,
    pub emr_id: EmrId,
    pub provider_id: ProviderId,
}
impl From<CompositeKey> for EmrHeader {
    fn from(value: CompositeKey) -> Self {
        Self {
            user_id: value.0,
            provider_id: value.1,
            emr_id: value.2,
        }
    }
}
