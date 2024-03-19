use candid::CandidType;
use canister_common::{ common::Id, from };
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct EmrListProviderRequest {
    pub page: u64,
    pub limit: u8,
}

#[derive(CandidType, Deserialize)]
pub struct EmrListProviderResponse {
    ids: Vec<Id>,
}
from!(EmrListProviderResponse: Vec<Id> as value {
    ids: value
});
