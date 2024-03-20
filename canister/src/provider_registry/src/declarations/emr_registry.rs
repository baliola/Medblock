// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdRequest {
  pub provider_id: String,
  pub user_id: String,
  pub emr_id: String,
}

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdResponse { pub emr: Vec<(String,String,)> }

pub struct EmrRegistry(pub Principal);
impl EmrRegistry {
  pub async fn dummy(&self) -> Result<()> {
    ic_cdk::call(self.0, "dummy", ()).await
  }
  pub async fn read_emr_by_id(&self, arg0: ReadEmrByIdRequest) -> Result<
    (ReadEmrByIdResponse,)
  > { ic_cdk::call(self.0, "read_emr_by_id", (arg0,)).await }
}
pub const CANISTER_ID : Principal = Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 4, 1, 1]); // br5f7-7uaaa-aaaaa-qaaca-cai
pub const emr_registry : EmrRegistry = EmrRegistry(CANISTER_ID);