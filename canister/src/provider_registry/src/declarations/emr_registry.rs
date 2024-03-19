// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

pub struct EmrRegistry(pub Principal);
impl EmrRegistry {
  pub async fn dummy(&self) -> Result<()> {
    ic_cdk::call(self.0, "dummy", ()).await
  }
}
pub const CANISTER_ID : Principal = Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 4, 1, 1]); // br5f7-7uaaa-aaaaa-qaaca-cai
pub const emr_registry : EmrRegistry = EmrRegistry(CANISTER_ID);