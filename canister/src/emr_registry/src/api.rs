use candid::CandidType;
use canister_common::{ common::{ EmrId, ProviderId, RawEmr, UserId }, from };
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdRequest {
    pub user_id: UserId,
    pub provider_id: ProviderId,
    pub emr_id: EmrId,
}

impl ReadEmrByIdRequest {
    pub fn to_read_key(self) -> crate::registry::key::EmrKey {
        crate::registry::key::EmrKey
            ::new()
            .with_user(self.user_id)
            .with_provider(self.provider_id)
            .with_emr_id(self.emr_id)
    }
}

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdResponse {
    pub emr: RawEmr,
}

from!(ReadEmrByIdResponse: RawEmr as raw {
    emr : raw
});
