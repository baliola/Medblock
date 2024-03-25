use candid::CandidType;
use canister_common::{ common::{ EmrBody, EmrHeaderWithBody, EmrId, ProviderId, UserId }, from };
use serde::Deserialize;

use crate::registry::key;


pub use crate::header;

use self::header::Header;

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdRequest {
    pub user_id: UserId,
    pub provider_id: ProviderId,
    pub emr_id: EmrId,
}

impl ReadEmrByIdRequest {
    pub fn to_read_key(self) -> key::EmrKey {
        key::EmrKey
            ::new()
            .with_user(self.user_id)
            .with_provider(self.provider_id)
            .with_emr_id(self.emr_id)
    }
}

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdResponse {
    pub emr: EmrHeaderWithBody,
}

from!(ReadEmrByIdResponse: EmrHeaderWithBody as raw {
    emr : raw
});

#[derive(CandidType, Deserialize)]
pub struct CreateEmrRequest {
    pub user_id: UserId,
    pub provider_id: ProviderId,
    pub emr: EmrBody,
}

impl CreateEmrRequest {
    pub fn to_args(self, emr_id: EmrId) -> (key::AddEmrKey, EmrBody) {
        let key = key::AddEmrKey
            ::new()
            .with_user(self.user_id)
            .with_provider(self.provider_id)
            .with_emr_id(emr_id);

        (key, self.emr)
    }
}

#[derive(CandidType, Deserialize)]
pub struct CreateEmrResponse {
    pub header: Header,
}

from!(CreateEmrResponse: Header as header {
    header : header
});

#[derive(CandidType, Deserialize)]
pub struct UpdateEmrRequest {
    pub header: Header,
    pub fields: EmrBody,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateEmrResponse {
    pub header: Header,
}

from!(UpdateEmrResponse: Header as header {
    header : header
});

#[derive(CandidType, Deserialize)]
pub struct RemoveEmrRequest {
    pub header: Header,
}

#[derive(CandidType, Deserialize)]
pub struct RemoveEmrResponse {
    status: bool,
}

impl RemoveEmrResponse {
    pub fn new(status: bool) -> Self {
        Self { status }
    }
}

from!(RemoveEmrResponse: bool as status {
    status : status
});
