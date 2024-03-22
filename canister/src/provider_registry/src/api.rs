use candid::CandidType;
use canister_common::{
    common::{ AsciiRecordsKey, EmrBody, EmrFragment, ProviderId, UserId },
    from,
};
use serde::Deserialize;

use crate::declarations::emr_registry::{ self, CreateEmrRequest, CreateEmrResponse };

#[derive(CandidType, Deserialize)]
pub struct IssueEmrRequest {
    pub emr: EmrBody,
    pub user_id: UserId,
}

impl IssueEmrRequest {
    pub fn to_args(self, provider_id: ProviderId) -> CreateEmrRequest {
        let emr = self.emr
            .into_inner()
            .into_iter()
            .map(|fragment| crate::declarations::emr_registry::EmrFragment {
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
    // it's fine to use the auto generated types for this as we dont use it for anyhting else, also
    // because eventually candid intrepret this as a text record, so doing serialization again just introduce
    // unnecessary overhead.
    pub emr_header: crate::declarations::emr_registry::EmrHeader,
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

#[derive(CandidType, Deserialize)]
pub struct RegisternewProviderResponse {
    // empty for now
}

#[derive(CandidType, Deserialize)]
pub struct UpdateEmrRequest {
    pub fields: Vec<EmrFragment>,
    pub header: canister_common::common::EmrHeader,
}

impl UpdateEmrRequest {
    pub fn to_args(self) -> crate::declarations::emr_registry::UpdateEmrRequest {
        let fields = self.fields
            .into_iter()
            .map(|fragment| crate::declarations::emr_registry::EmrFragment {
                key: fragment.key.to_string(),
                value: fragment.value,
            })
            .collect::<Vec<_>>();

        let header = crate::declarations::emr_registry::EmrHeader {
            provider_id: self.header.provider_id.to_string(),
            user_id: self.header.user_id.to_string(),
            emr_id: self.header.emr_id.to_string(),
        };

        crate::declarations::emr_registry::UpdateEmrRequest {
            fields,
            header,
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct UpdateEmrResponse {
    // empty for now
}
