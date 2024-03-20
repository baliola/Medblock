use candid::CandidType;
use canister_common::{ common::{ EmrId, ProviderId, RawEmr, UserId }, from };
use serde::Deserialize;

use crate::{ header::{ EmrHeader, EmrHeaderWithBody }, registry::key };

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
    pub emr: RawEmr,
}

impl CreateEmrRequest {
    pub fn to_args(self, emr_id: EmrId) -> (key::AddEmrKey, RawEmr) {
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
    header: EmrHeader,
}

from!(CreateEmrResponse: EmrHeader as header {
    header : header
});
