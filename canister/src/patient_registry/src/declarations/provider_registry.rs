// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct EmrListProviderRequest { pub page: u64, pub limit: u8 }

#[derive(CandidType, Deserialize)]
pub struct EmrListProviderResponse { pub ids: Vec<String> }

#[derive(CandidType, Deserialize)]
pub struct EmrFragment { pub key: String, pub value: String }

#[derive(CandidType, Deserialize)]
pub struct IssueEmrRequest { pub emr: Vec<EmrFragment>, pub user_id: String }

#[derive(CandidType, Deserialize)]
pub struct Header {
  pub provider_id: String,
  pub user_id: String,
  pub emr_id: String,
  pub registry_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct IssueEmrResponse { pub emr_header: Header }

#[derive(CandidType, Deserialize)]
pub struct PingResult {
  pub patient_registry_status: bool,
  pub emr_registry_status: bool,
}

#[derive(CandidType, Deserialize)]
pub struct RegisternewProviderRequest {
  pub provider_principal: Principal,
  pub display_name: String,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterNewProviderRet {}

#[derive(CandidType, Deserialize)]
pub struct EmrHeader {
  pub provider_id: String,
  pub user_id: String,
  pub emr_id: String,
  pub registry_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateEmrRequest {
  pub fields: Vec<EmrFragment>,
  pub header: EmrHeader,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateEmrRet {}

pub struct ProviderRegistry(pub Principal);
impl ProviderRegistry {
  pub async fn emr_list_provider(&self, arg0: EmrListProviderRequest) -> Result<
    (EmrListProviderResponse,)
  > { ic_cdk::call(self.0, "emr_list_provider", (arg0,)).await }
  pub async fn issue_emr(&self, arg0: IssueEmrRequest) -> Result<
    (IssueEmrResponse,)
  > { ic_cdk::call(self.0, "issue_emr", (arg0,)).await }
  pub async fn metrics(&self) -> Result<(String,)> {
    ic_cdk::call(self.0, "metrics", ()).await
  }
  pub async fn ping(&self) -> Result<(PingResult,)> {
    ic_cdk::call(self.0, "ping", ()).await
  }
  pub async fn register_new_provider(
    &self,
    arg0: RegisternewProviderRequest,
  ) -> Result<(RegisterNewProviderRet,)> {
    ic_cdk::call(self.0, "register_new_provider", (arg0,)).await
  }
  pub async fn update_emr(&self, arg0: UpdateEmrRequest) -> Result<
    (UpdateEmrRet,)
  > { ic_cdk::call(self.0, "update_emr", (arg0,)).await }
}
pub const CANISTER_ID : Principal = Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 2, 1, 1]); // bd3sg-teaaa-aaaaa-qaaba-cai
pub const provider_registry : ProviderRegistry = ProviderRegistry(CANISTER_ID);