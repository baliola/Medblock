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
pub struct EmrHeaderWithBody {
  pub body: Vec<EmrFragment>,
  pub header: EmrHeader,
}

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdResponse { pub emr: EmrHeaderWithBody }

#[derive(CandidType, Deserialize)]
pub struct RemoveEmrRequest { pub header: EmrHeader }

#[derive(CandidType, Deserialize)]
pub struct RemoveEmrResponse { pub status: bool }

#[derive(CandidType, Deserialize)]
pub struct UpdateEmrRequest {
  pub fields: Vec<EmrFragment>,
  pub header: EmrHeader,
}

pub struct EmrRegistry(pub Principal);
impl EmrRegistry {
  pub async fn create_emr(&self, arg0: CreateEmrRequest) -> Result<
    (CreateEmrResponse,)
  > { ic_cdk::call(self.0, "create_emr", (arg0,)).await }
  pub async fn ping(&self) -> Result<()> {
    ic_cdk::call(self.0, "ping", ()).await
  }
  pub async fn read_emr_by_id(&self, arg0: EmrHeader) -> Result<
    (ReadEmrByIdResponse,)
  > { ic_cdk::call(self.0, "read_emr_by_id", (arg0,)).await }
  pub async fn remove_emr(&self, arg0: RemoveEmrRequest) -> Result<
    (RemoveEmrResponse,)
  > { ic_cdk::call(self.0, "remove_emr", (arg0,)).await }
  pub async fn update_emr(&self, arg0: UpdateEmrRequest) -> Result<
    (RemoveEmrRequest,)
  > { ic_cdk::call(self.0, "update_emr", (arg0,)).await }
}
pub const CANISTER_ID : Principal = Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 1, 1, 1]); // bkyz2-fmaaa-aaaaa-qaaaq-cai
pub const emr_registry : EmrRegistry = EmrRegistry(CANISTER_ID);