use candid::{ CandidType, Principal };
use canister_common::{
    common::{ EmrHeader, EmrId, ProviderId, UserId, H256 },
    stable::{ Candid, Stable },
};
use serde::Deserialize;

use crate::registry::NIK;

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdRequest {
    pub provider_id: ProviderId,
    pub emr_id: EmrId,
    pub registry_id: Principal,
}

impl ReadEmrByIdRequest {
    pub fn to_args(self, user_id: UserId) -> crate::declarations::emr_registry::ReadEmrByIdRequest {
        crate::declarations::emr_registry::ReadEmrByIdRequest {
            provider_id: self.provider_id.to_string(),
            emr_id: self.emr_id.to_string(),
            user_id: user_id.to_string(),
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct EmrListPatientRequest {
    pub limit: u8,
    pub page: u8,
}

#[derive(CandidType, Deserialize)]
pub struct EmrListPatientResponse {
    emrs: Vec<EmrHeader>,
}

impl From<Vec<Stable<EmrHeader>>> for EmrListPatientResponse {
    fn from(value: Vec<Stable<EmrHeader>>) -> Self {
        Self {
            emrs: value
                .into_iter()
                .map(|x| x.into_inner())
                .collect(),
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct RegisterPatientRequest {
    pub nik: H256,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterProviderResponse {
    // empty for now
}

#[derive(CandidType, Deserialize)]
pub struct PingResult {
    pub emr_registry_status: bool,
}

#[derive(CandidType, Deserialize)]
pub struct IssueRequest {
    pub header: EmrHeader,
}
