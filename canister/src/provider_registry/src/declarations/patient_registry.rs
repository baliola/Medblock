// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct EmrHeader {
  pub provider_id: String,
  pub user_id: String,
  pub emr_id: String,
  pub registry_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct IssueRequest { pub header: EmrHeader }

#[derive(CandidType, Deserialize)]
pub struct PingResult { pub emr_registry_status: bool }

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdRequest {
  pub provider_id: String,
  pub emr_id: String,
  pub registry_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct EmrFragment { pub key: String, pub value: String }

#[derive(CandidType, Deserialize)]
pub struct EmrHeaderWithBody {
  pub body: Vec<EmrFragment>,
  pub header: EmrHeader,
}

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdResponse { pub emr: EmrHeaderWithBody }

#[derive(CandidType, Deserialize)]
pub struct RegisterPatientRequest { pub nik: String }

pub struct PatientRegistry(pub Principal);
impl PatientRegistry {
  pub async fn metrics(&self) -> Result<(String,)> {
    ic_cdk::call(self.0, "metrics", ()).await
  }
  pub async fn notify_issued(&self, arg0: IssueRequest) -> Result<()> {
    ic_cdk::call(self.0, "notify_issued", (arg0,)).await
  }
  pub async fn ping(&self) -> Result<(PingResult,)> {
    ic_cdk::call(self.0, "ping", ()).await
  }
  pub async fn read_emr_by_id(&self, arg0: ReadEmrByIdRequest) -> Result<
    (ReadEmrByIdResponse,)
  > { ic_cdk::call(self.0, "read_emr_by_id", (arg0,)).await }
  pub async fn register_patient(&self, arg0: RegisterPatientRequest) -> Result<
    ()
  > { ic_cdk::call(self.0, "register_patient", (arg0,)).await }
}
pub const CANISTER_ID : Principal = Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 2, 1, 1]); // bd3sg-teaaa-aaaaa-qaaba-cai
pub const patient_registry : PatientRegistry = PatientRegistry(CANISTER_ID);