use candid::CandidType;
use canister_common::{ common::{ AsciiRecordsKey, EmrBody, ProviderId, UserId }, from };
use serde::Deserialize;

use crate::declarations::emr_registry::{
    CreateEmrRequest,
    CreateEmrResponse,
    EmrFragment,
    EmrHeader,
};

#[derive(CandidType, Deserialize)]
pub struct IssueEmrRequest {
    pub emr: EmrBody,
    pub user_id: UserId,
}

impl IssueEmrRequest {
    pub fn to_create_emr_args(self, provider_id: ProviderId) -> CreateEmrRequest {
        let emr = self.emr
            .into_inner()
            .into_iter()
            .map(|fragment| EmrFragment {
                key: fragment.key.to_string(),
                value: fragment.value,
            })
            .collect::<Vec<_>>();

        CreateEmrRequest {
            emr,
            provider_id: provider_id.to_string(),
            user_id: self.user_id.to_string(),
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct IssueEmrResponse {
    pub emr_header: EmrHeader,
}

impl From<CreateEmrResponse> for IssueEmrResponse {
    fn from(response: CreateEmrResponse) -> Self {
        IssueEmrResponse {
            emr_header: response.header,
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct PingResult {
    pub emr_registry_status: bool,
    pub patient_registry_status: bool,
}

#[derive(CandidType, Deserialize)]
pub struct RegisternewProviderRequest {
    pub provider_principal: ic_principal::Principal,
    pub display_name: AsciiRecordsKey<64>,
}
