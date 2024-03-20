// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct EmrFragment { pub key: String, pub value: String }

#[derive(CandidType, Deserialize)]
pub struct CreateEmrRequest {
  pub emr: Vec<EmrFragment>,
  pub provider_id: String,
  pub user_id: String,
}

#[derive(CandidType, Deserialize)]
pub struct EmrHeader {
  pub provider_id: String,
  pub user_id: String,
  pub emr_id: String,
}

#[derive(CandidType, Deserialize)]
pub struct CreateEmrResponse { pub header: EmrHeader }

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdResponse { pub emr: Vec<EmrFragment> }

pub struct EmrRegistry(pub Principal);
impl EmrRegistry {
  pub async fn create_emr(&self, arg0: CreateEmrRequest) -> Result<
    (CreateEmrResponse,)
  > { ic_cdk::call(self.0, "create_emr", (arg0,)).await }
  pub async fn dummy(&self) -> Result<()> {
    ic_cdk::call(self.0, "dummy", ()).await
  }
  pub async fn read_emr_by_id(&self, arg0: EmrHeader) -> Result<
    (ReadEmrByIdResponse,)
  > { ic_cdk::call(self.0, "read_emr_by_id", (arg0,)).await }
}
pub const CANISTER_ID : Principal = Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 4, 1, 1]); // br5f7-7uaaa-aaaaa-qaaca-cai
pub const emr_registry : EmrRegistry = EmrRegistry(CANISTER_ID);