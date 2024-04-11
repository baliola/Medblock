// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct AuthorizedCallerRequest { pub caller: Principal }

#[derive(CandidType, Deserialize)]
pub struct EmrListProviderRequest { pub page: u64, pub limit: u8 }

#[derive(CandidType, Deserialize)]
pub struct EmrListProviderResponse { pub ids: Vec<String> }

#[derive(CandidType, Deserialize)]
pub struct StatusRequest {
  pub memory_size: bool,
  pub cycles: bool,
  pub heap_memory_size: bool,
}

#[derive(CandidType, Deserialize)]
pub enum MetricsGranularity {
  #[serde(rename="hourly")]
  Hourly,
  #[serde(rename="daily")]
  Daily,
}

#[derive(CandidType, Deserialize)]
pub struct GetMetricsParameters {
  pub dateToMillis: candid::Nat,
  pub granularity: MetricsGranularity,
  pub dateFromMillis: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct MetricsRequest { pub parameters: GetMetricsParameters }

#[derive(CandidType, Deserialize)]
pub struct GetLogMessagesFilter {
  pub analyzeCount: u32,
  pub messageRegex: Option<String>,
  pub messageContains: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct GetLogMessagesParameters {
  pub count: u32,
  pub filter: Option<GetLogMessagesFilter>,
  pub fromTimeNanos: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct GetLatestLogMessagesParameters {
  pub upToTimeNanos: Option<u64>,
  pub count: u32,
  pub filter: Option<GetLogMessagesFilter>,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterLogRequest {
  #[serde(rename="getMessagesInfo")]
  GetMessagesInfo,
  #[serde(rename="getMessages")]
  GetMessages(GetLogMessagesParameters),
  #[serde(rename="getLatestMessages")]
  GetLatestMessages(GetLatestLogMessagesParameters),
}

#[derive(CandidType, Deserialize)]
pub struct GetInformationRequest {
  pub status: Option<StatusRequest>,
  pub metrics: Option<MetricsRequest>,
  pub logs: Option<CanisterLogRequest>,
  pub version: bool,
}

#[derive(CandidType, Deserialize)]
pub struct StatusResponse {
  pub memory_size: Option<u64>,
  pub cycles: Option<u64>,
  pub heap_memory_size: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct HourlyMetricsData {
  pub updateCalls: Vec<u64>,
  pub canisterHeapMemorySize: Vec<u64>,
  pub canisterCycles: Vec<u64>,
  pub canisterMemorySize: Vec<u64>,
  pub timeMillis: candid::Int,
}

#[derive(CandidType, Deserialize)]
pub struct NumericEntity {
  pub avg: u64,
  pub max: u64,
  pub min: u64,
  pub first: u64,
  pub last: u64,
}

#[derive(CandidType, Deserialize)]
pub struct DailyMetricsData {
  pub updateCalls: u64,
  pub canisterHeapMemorySize: NumericEntity,
  pub canisterCycles: NumericEntity,
  pub canisterMemorySize: NumericEntity,
  pub timeMillis: candid::Int,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterMetricsData {
  #[serde(rename="hourly")]
  Hourly(Vec<HourlyMetricsData>),
  #[serde(rename="daily")]
  Daily(Vec<DailyMetricsData>),
}

#[derive(CandidType, Deserialize)]
pub struct CanisterMetrics { pub data: CanisterMetricsData }

#[derive(CandidType, Deserialize)]
pub struct MetricsResponse { pub metrics: Option<CanisterMetrics> }

#[derive(CandidType, Deserialize)]
pub enum CanisterLogFeature {
  #[serde(rename="filterMessageByContains")]
  FilterMessageByContains,
  #[serde(rename="filterMessageByRegex")]
  FilterMessageByRegex,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterLogMessagesInfo {
  pub features: Vec<Option<CanisterLogFeature>>,
  pub lastTimeNanos: Option<u64>,
  pub count: u32,
  pub firstTimeNanos: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct LogMessageData { pub timeNanos: u64, pub message: String }

#[derive(CandidType, Deserialize)]
pub struct CanisterLogMessages {
  pub data: Vec<LogMessageData>,
  pub lastAnalyzedMessageTimeNanos: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterLogResponse {
  #[serde(rename="messagesInfo")]
  MessagesInfo(CanisterLogMessagesInfo),
  #[serde(rename="messages")]
  Messages(CanisterLogMessages),
}

#[derive(CandidType, Deserialize)]
pub struct GetInformationResponse {
  pub status: Option<StatusResponse>,
  pub metrics: Option<MetricsResponse>,
  pub logs: Option<CanisterLogResponse>,
  pub version: Option<candid::Nat>,
}

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
pub struct SuspendRequest { pub principal: Principal }

#[derive(CandidType, Deserialize)]
pub enum CollectMetricsRequestType {
  #[serde(rename="force")]
  Force,
  #[serde(rename="normal")]
  Normal,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateInformationRequest {
  pub metrics: Option<CollectMetricsRequestType>,
}

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
  pub async fn add_authorized_metrics_collector(
    &self,
    arg0: AuthorizedCallerRequest,
  ) -> Result<()> {
    ic_cdk::call(self.0, "add_authorized_metrics_collector", (arg0,)).await
  }
  pub async fn emr_list_provider(&self, arg0: EmrListProviderRequest) -> Result<
    (EmrListProviderResponse,)
  > { ic_cdk::call(self.0, "emr_list_provider", (arg0,)).await }
  pub async fn get_canistergeek_information(
    &self,
    arg0: GetInformationRequest,
  ) -> Result<(GetInformationResponse,)> {
    ic_cdk::call(self.0, "getCanistergeekInformation", (arg0,)).await
  }
  pub async fn get_trusted_origins(&self) -> Result<(Vec<String>,)> {
    ic_cdk::call(self.0, "get_trusted_origins", ()).await
  }
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
  pub async fn remove_authorized_metrics_collector(
    &self,
    arg0: AuthorizedCallerRequest,
  ) -> Result<()> {
    ic_cdk::call(self.0, "remove_authorized_metrics_collector", (arg0,)).await
  }
  pub async fn suspend_provider(&self, arg0: SuspendRequest) -> Result<()> {
    ic_cdk::call(self.0, "suspend_provider", (arg0,)).await
  }
  pub async fn unsuspend_provider(&self, arg0: SuspendRequest) -> Result<()> {
    ic_cdk::call(self.0, "unsuspend_provider", (arg0,)).await
  }
  pub async fn update_canistergeek_information(
    &self,
    arg0: UpdateInformationRequest,
  ) -> Result<()> {
    ic_cdk::call(self.0, "updateCanistergeekInformation", (arg0,)).await
  }
  pub async fn update_emr(&self, arg0: UpdateEmrRequest) -> Result<
    (UpdateEmrRet,)
  > { ic_cdk::call(self.0, "update_emr", (arg0,)).await }
  pub async fn update_emr_registry_principal(
    &self,
    arg0: SuspendRequest,
  ) -> Result<()> {
    ic_cdk::call(self.0, "update_emr_registry_principal", (arg0,)).await
  }
  pub async fn update_patient_registry_principal(
    &self,
    arg0: SuspendRequest,
  ) -> Result<()> {
    ic_cdk::call(self.0, "update_patient_registry_principal", (arg0,)).await
  }
}
pub const CANISTER_ID : Principal = Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 3, 1, 1]); // be2us-64aaa-aaaaa-qaabq-cai
pub const provider_registry : ProviderRegistry = ProviderRegistry(CANISTER_ID);