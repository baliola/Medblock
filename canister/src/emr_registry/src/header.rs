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

impl EmrHeader {
    pub fn new(user_id: UserId, emr_id: EmrId, provider_id: ProviderId) -> Self {
        Self { user_id, emr_id, provider_id }
    }
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
