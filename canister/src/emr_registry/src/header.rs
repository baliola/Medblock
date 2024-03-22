use candid::CandidType;
use canister_common::{ common::{ EmrBody, EmrHeader, EmrId, ProviderId, UserId }, deref, from };
use serde::Deserialize;

use crate::{ key::{ CompositeKey }, registry::key::{ EmrKey, PartialUpdateKey } };

#[derive(Debug, Deserialize, CandidType, PartialEq, Eq)]
pub struct Header(EmrHeader);
deref!(Header: EmrHeader);
from!(Header: EmrHeader);
impl Into<EmrHeader> for Header {
    fn into(self) -> EmrHeader {
        self.0
    }
}

impl Header {
    pub fn into_inner(self) -> EmrHeader {
        self.0
    }

    pub fn new(user_id: UserId, provider_id: ProviderId, emr_id: EmrId) -> Self {
        Header(EmrHeader {
            user_id,
            provider_id,
            emr_id,
        })
    }

    pub fn to_emr_key(self) -> EmrKey {
        EmrKey::new()
            .with_user(self.0.user_id)
            .with_provider(self.0.provider_id)
            .with_emr_id(self.0.emr_id)
    }

    pub fn to_partial_update_key(self) -> PartialUpdateKey {
        PartialUpdateKey::new()
            .with_user(self.0.user_id)
            .with_provider(self.0.provider_id)
            .with_emr_id(self.0.emr_id)
    }
}

impl From<CompositeKey> for Header {
    fn from(key: CompositeKey) -> Self {
        Header(EmrHeader {
            user_id: UserId::from(key.0),
            provider_id: ProviderId::from(key.1),
            emr_id: EmrId::from(key.2),
        })
    }
}
