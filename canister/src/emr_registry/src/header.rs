use candid::CandidType;
use canister_common::{ common::{ EmrId, ProviderId, RawEmr, UserId }, from };
use serde::Deserialize;

use crate::{
    key::{ CompositeKey, CompositeKeyBuilder, Known },
    registry::key::{ EmrKey, PartialUpdateKey },
};

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

    pub fn to_emr_key(self) -> EmrKey {
        EmrKey::new()
            .with_user(self.user_id)
            .with_provider(self.provider_id)
            .with_emr_id(self.emr_id)
    }

    pub fn to_partial_update_key(self) -> PartialUpdateKey {
        PartialUpdateKey::new()
            .with_user(self.user_id)
            .with_provider(self.provider_id)
            .with_emr_id(self.emr_id)
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

#[derive(Debug, Deserialize, CandidType, PartialEq, Eq)]
pub struct EmrHeaderWithBody {
    pub header: EmrHeader,
    pub body: RawEmr,
}

impl EmrHeaderWithBody {
    pub fn new(header: EmrHeader, body: RawEmr) -> Self {
        Self { header, body }
    }

    pub fn to_header(self) -> EmrHeader {
        self.header
    }

    pub fn into_inner_body(self) -> RawEmr {
        self.body
    }
}
