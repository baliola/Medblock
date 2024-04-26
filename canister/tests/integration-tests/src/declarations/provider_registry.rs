#![allow(warnings, unused_imports)]
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;
#[derive(CandidType, Deserialize)]
pub struct AuthorizedCallerRequest {
    pub caller: Principal,
}
#[derive(CandidType, Deserialize)]
pub struct EmrListProviderRequest {
    pub page: u64,
    pub limit: u8,
}
#[derive(CandidType, Deserialize)]
pub struct EmrListProviderResponse {
    pub ids: Vec<String>,
}
#[derive(CandidType, Deserialize)]
pub struct StatusRequest {
    pub memory_size: bool,
    pub cycles: bool,
    pub heap_memory_size: bool,
}
#[derive(CandidType, Deserialize)]
pub enum MetricsGranularity {
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "daily")]
    Daily,
}
#[derive(CandidType, Deserialize)]
pub struct GetMetricsParameters {
    pub dateToMillis: candid::Nat,
    pub granularity: MetricsGranularity,
    pub dateFromMillis: candid::Nat,
}
#[derive(CandidType, Deserialize)]
pub struct MetricsRequest {
    pub parameters: GetMetricsParameters,
}
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
    #[serde(rename = "getMessagesInfo")]
    GetMessagesInfo,
    #[serde(rename = "getMessages")]
    GetMessages(GetLogMessagesParameters),
    #[serde(rename = "getLatestMessages")]
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
    #[serde(rename = "hourly")]
    Hourly(Vec<HourlyMetricsData>),
    #[serde(rename = "daily")]
    Daily(Vec<DailyMetricsData>),
}
#[derive(CandidType, Deserialize)]
pub struct CanisterMetrics {
    pub data: CanisterMetricsData,
}
#[derive(CandidType, Deserialize)]
pub struct MetricsResponse {
    pub metrics: Option<CanisterMetrics>,
}
#[derive(CandidType, Deserialize)]
pub enum CanisterLogFeature {
    #[serde(rename = "filterMessageByContains")]
    FilterMessageByContains,
    #[serde(rename = "filterMessageByRegex")]
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
pub struct LogMessageData {
    pub timeNanos: u64,
    pub message: String,
}
#[derive(CandidType, Deserialize)]
pub struct CanisterLogMessages {
    pub data: Vec<LogMessageData>,
    pub lastAnalyzedMessageTimeNanos: Option<u64>,
}
#[derive(CandidType, Deserialize)]
pub enum CanisterLogResponse {
    #[serde(rename = "messagesInfo")]
    MessagesInfo(CanisterLogMessagesInfo),
    #[serde(rename = "messages")]
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
pub struct GetProviderBatchRequest {
    pub ids: Vec<String>,
}
#[derive(CandidType, Deserialize)]
pub enum Status {
    Active,
    Suspended,
}
#[derive(CandidType, Deserialize)]
pub struct V1 {
    pub updated_at: u64,
    pub internal_id: String,
    pub display_name: String,
    pub session: u64,
    pub address: String,
    pub registered_at: u64,
    pub activation_status: Status,
}
#[derive(CandidType, Deserialize)]
pub enum Provider {
    V1(V1),
}
#[derive(CandidType, Deserialize)]
pub struct GetProviderBatchResponse {
    pub providers: Vec<Provider>,
}
#[derive(CandidType, Deserialize)]
pub struct ProviderInfoRequest {
    pub provider: Vec<Principal>,
}
#[derive(CandidType, Deserialize)]
pub struct ProviderInfoResponse {
    pub providers: Vec<Provider>,
}
#[derive(CandidType, Deserialize)]
pub struct EmrFragment {
    pub key: String,
    pub value: String,
}
#[derive(CandidType, Deserialize)]
pub struct IssueEmrRequest {
    pub emr: Vec<EmrFragment>,
    pub user_id: String,
}
#[derive(CandidType, Deserialize)]
pub struct Header {
    pub provider_id: String,
    pub user_id: String,
    pub emr_id: String,
    pub registry_id: Principal,
}
#[derive(CandidType, Deserialize)]
pub struct IssueEmrResponse {
    pub emr_header: Header,
}
#[derive(CandidType, Deserialize)]
pub struct PingResult {
    pub patient_registry_status: bool,
    pub emr_registry_status: bool,
}
#[derive(CandidType, Deserialize)]
pub struct RegisternewProviderRequest {
    pub provider_principal: Principal,
    pub display_name: String,
    pub address: String,
}
#[derive(CandidType, Deserialize)]
pub struct RegisterNewProviderRet {}
#[derive(CandidType, Deserialize)]
pub struct SuspendRequest {
    pub principal: Principal,
}
#[derive(CandidType, Deserialize)]
pub enum CollectMetricsRequestType {
    #[serde(rename = "force")]
    Force,
    #[serde(rename = "normal")]
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
    pub async fn emr_list_provider(
        &self,
        arg0: EmrListProviderRequest,
    ) -> Result<(EmrListProviderResponse,)> {
        ic_cdk::call(self.0, "emr_list_provider", (arg0,)).await
    }
    pub async fn get_canistergeek_information(
        &self,
        arg0: GetInformationRequest,
    ) -> Result<(GetInformationResponse,)> {
        ic_cdk::call(self.0, "getCanistergeekInformation", (arg0,)).await
    }
    pub async fn get_provider_batch(
        &self,
        arg0: GetProviderBatchRequest,
    ) -> Result<(GetProviderBatchResponse,)> {
        ic_cdk::call(self.0, "get_provider_batch", (arg0,)).await
    }
    pub async fn get_provider_info_with_principal(
        &self,
        arg0: ProviderInfoRequest,
    ) -> Result<(ProviderInfoResponse,)> {
        ic_cdk::call(self.0, "get_provider_info_with_principal", (arg0,)).await
    }
    pub async fn get_trusted_origins(&self) -> Result<(Vec<String>,)> {
        ic_cdk::call(self.0, "get_trusted_origins", ()).await
    }
    pub async fn issue_emr(&self, arg0: IssueEmrRequest) -> Result<(IssueEmrResponse,)> {
        ic_cdk::call(self.0, "issue_emr", (arg0,)).await
    }
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
    pub async fn update_emr(&self, arg0: UpdateEmrRequest) -> Result<(UpdateEmrRet,)> {
        ic_cdk::call(self.0, "update_emr", (arg0,)).await
    }
    pub async fn update_emr_registry_principal(&self, arg0: SuspendRequest) -> Result<()> {
        ic_cdk::call(self.0, "update_emr_registry_principal", (arg0,)).await
    }
    pub async fn update_patient_registry_principal(&self, arg0: SuspendRequest) -> Result<()> {
        ic_cdk::call(self.0, "update_patient_registry_principal", (arg0,)).await
    }
}
pub const CANISTER_ID: Principal = Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 3, 1, 1]);
pub const provider_registry: ProviderRegistry = ProviderRegistry(CANISTER_ID);
pub mod pocket_ic_bindings {
    use super::*;
    use pocket_ic;
    fn call_pocket_ic<
        R: candid::CandidType + serde::de::DeserializeOwned,
        A: candid::CandidType + serde::de::DeserializeOwned,
    >(
        s: &pocket_ic::PocketIc,
        f: impl FnOnce(
            &pocket_ic::PocketIc,
            ic_principal::Principal,
            ic_principal::Principal,
            &str,
            Vec<u8>,
        ) -> std::result::Result<pocket_ic::WasmResult, pocket_ic::UserError>,
        id: ic_principal::Principal,
        sender: ic_principal::Principal,
        method: &str,
        payload: A,
    ) -> std::result::Result<R, pocket_ic::UserError> {
        use candid::Decode;
        use candid::Encode;
        let args = Encode!(&payload).unwrap();
        let result = f(s, id, sender, method, args);
        match result {
            Ok(r) => match r {
                pocket_ic::WasmResult::Reply(vec) => Ok(Decode!(vec.as_slice(), R).unwrap()),
                pocket_ic::WasmResult::Reject(e) => panic!("Error: {:?}", e),
            },
            Err(e) => Err(e),
        }
    }
    #[derive(Clone, Debug, Deserialize, CandidType, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Call {
        Query,
        Update,
    }
    pub struct ProviderRegistry(pub ic_principal::Principal);
    impl ProviderRegistry {
        pub fn add_authorized_metrics_collector(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: AuthorizedCallerRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "add_authorized_metrics_collector",
                payload,
            )
        }
        pub fn emr_list_provider(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: EmrListProviderRequest,
        ) -> std::result::Result<EmrListProviderResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "emr_list_provider",
                payload,
            )
        }
        pub fn get_canistergeek_information(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: GetInformationRequest,
        ) -> std::result::Result<GetInformationResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "get_canistergeek_information",
                payload,
            )
        }
        pub fn get_provider_batch(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: GetProviderBatchRequest,
        ) -> std::result::Result<GetProviderBatchResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "get_provider_batch",
                payload,
            )
        }
        pub fn get_provider_info_with_principal(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: ProviderInfoRequest,
        ) -> std::result::Result<ProviderInfoResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "get_provider_info_with_principal",
                payload,
            )
        }
        pub fn get_trusted_origins(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
        ) -> std::result::Result<Vec<String>, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = ();
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "get_trusted_origins",
                payload,
            )
        }
        pub fn issue_emr(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: IssueEmrRequest,
        ) -> std::result::Result<IssueEmrResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "issue_emr", payload)
        }
        pub fn metrics(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
        ) -> std::result::Result<String, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = ();
            call_pocket_ic(server, f, self.0.clone(), sender, "metrics", payload)
        }
        pub fn ping(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
        ) -> std::result::Result<PingResult, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = ();
            call_pocket_ic(server, f, self.0.clone(), sender, "ping", payload)
        }
        pub fn register_new_provider(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: RegisternewProviderRequest,
        ) -> std::result::Result<RegisterNewProviderRet, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "register_new_provider",
                payload,
            )
        }
        pub fn remove_authorized_metrics_collector(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: AuthorizedCallerRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "remove_authorized_metrics_collector",
                payload,
            )
        }
        pub fn suspend_provider(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: SuspendRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "suspend_provider",
                payload,
            )
        }
        pub fn unsuspend_provider(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: SuspendRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "unsuspend_provider",
                payload,
            )
        }
        pub fn update_canistergeek_information(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: UpdateInformationRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "update_canistergeek_information",
                payload,
            )
        }
        pub fn update_emr(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: UpdateEmrRequest,
        ) -> std::result::Result<UpdateEmrRet, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "update_emr", payload)
        }
        pub fn update_emr_registry_principal(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: SuspendRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "update_emr_registry_principal",
                payload,
            )
        }
        pub fn update_patient_registry_principal(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: SuspendRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "update_patient_registry_principal",
                payload,
            )
        }
    }
}
