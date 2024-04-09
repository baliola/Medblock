#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;
#[derive(CandidType, Deserialize)]
pub struct AuthorizedCallerRequest {
    pub caller: Principal,
}
#[derive(CandidType, Deserialize)]
pub struct ClaimConsentRequest {
    pub code: String,
}
#[derive(CandidType, Deserialize)]
pub struct ClaimConsentResponse {
    pub session_id: String,
}
#[derive(CandidType, Deserialize)]
pub struct EmrHeader {
    pub provider_id: String,
    pub user_id: String,
    pub emr_id: String,
    pub registry_id: Principal,
}
#[derive(CandidType, Deserialize)]
pub struct CreateConsentRequest {
    pub allowed: Vec<EmrHeader>,
}
#[derive(CandidType, Deserialize)]
pub struct EmrListPatientRequest {
    pub page: u8,
    pub limit: u8,
}
#[derive(CandidType, Deserialize)]
pub struct EmrListPatientResponse {
    pub emrs: Vec<EmrHeader>,
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
pub struct IssueRequest {
    pub header: EmrHeader,
}
#[derive(CandidType, Deserialize)]
pub struct PingResult {
    pub emr_registry_status: bool,
}
#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdRequest {
    pub provider_id: String,
    pub emr_id: String,
    pub registry_id: Principal,
}
#[derive(CandidType, Deserialize)]
pub struct EmrFragment {
    pub key: String,
    pub value: String,
}
#[derive(CandidType, Deserialize)]
pub struct EmrHeaderWithBody {
    pub body: Vec<EmrFragment>,
    pub header: EmrHeader,
}
#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdResponse {
    pub emr: EmrHeaderWithBody,
}
#[derive(CandidType, Deserialize)]
pub struct ReadEmrSessionRequest {
    pub session_id: String,
    pub args: ReadEmrByIdRequest,
}
#[derive(CandidType, Deserialize)]
pub struct RegisterPatientRequest {
    pub nik: String,
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
pub struct UpdateEmrRegistryRequest {
    pub principal: Principal,
}
pub struct PatientRegistry(pub Principal);
impl PatientRegistry {
    pub async fn add_authorized_metrics_collector(
        &self,
        arg0: AuthorizedCallerRequest,
    ) -> Result<()> {
        ic_cdk::call(self.0, "add_authorized_metrics_collector", (arg0,)).await
    }
    pub async fn claim_consent(
        &self,
        arg0: ClaimConsentRequest,
    ) -> Result<(ClaimConsentResponse,)> {
        ic_cdk::call(self.0, "claim_consent", (arg0,)).await
    }
    pub async fn create_consent(
        &self,
        arg0: CreateConsentRequest,
    ) -> Result<(ClaimConsentRequest,)> {
        ic_cdk::call(self.0, "create_consent", (arg0,)).await
    }
    pub async fn emr_list_patient(
        &self,
        arg0: EmrListPatientRequest,
    ) -> Result<(EmrListPatientResponse,)> {
        ic_cdk::call(self.0, "emr_list_patient", (arg0,)).await
    }
    pub async fn emr_list_with_session(
        &self,
        arg0: ClaimConsentResponse,
    ) -> Result<(EmrListPatientResponse,)> {
        ic_cdk::call(self.0, "emr_list_with_session", (arg0,)).await
    }
    pub async fn finish_session(&self, arg0: ClaimConsentResponse) -> Result<()> {
        ic_cdk::call(self.0, "finish_session", (arg0,)).await
    }
    pub async fn get_canistergeek_information(
        &self,
        arg0: GetInformationRequest,
    ) -> Result<(GetInformationResponse,)> {
        ic_cdk::call(self.0, "getCanistergeekInformation", (arg0,)).await
    }
    pub async fn metrics(&self) -> Result<(String,)> {
        ic_cdk::call(self.0, "metrics", ()).await
    }
    pub async fn notify_issued(&self, arg0: IssueRequest) -> Result<()> {
        ic_cdk::call(self.0, "notify_issued", (arg0,)).await
    }
    pub async fn ping(&self) -> Result<(PingResult,)> {
        ic_cdk::call(self.0, "ping", ()).await
    }
    pub async fn read_emr_by_id(&self, arg0: ReadEmrByIdRequest) -> Result<(ReadEmrByIdResponse,)> {
        ic_cdk::call(self.0, "read_emr_by_id", (arg0,)).await
    }
    pub async fn read_emr_with_session(
        &self,
        arg0: ReadEmrSessionRequest,
    ) -> Result<(ReadEmrByIdResponse,)> {
        ic_cdk::call(self.0, "read_emr_with_session", (arg0,)).await
    }
    pub async fn register_patient(&self, arg0: RegisterPatientRequest) -> Result<()> {
        ic_cdk::call(self.0, "register_patient", (arg0,)).await
    }
    pub async fn remove_authorized_metrics_collector(
        &self,
        arg0: AuthorizedCallerRequest,
    ) -> Result<()> {
        ic_cdk::call(self.0, "remove_authorized_metrics_collector", (arg0,)).await
    }
    pub async fn revoke_consent(&self, arg0: ClaimConsentRequest) -> Result<()> {
        ic_cdk::call(self.0, "revoke_consent", (arg0,)).await
    }
    pub async fn update_canistergeek_information(
        &self,
        arg0: UpdateInformationRequest,
    ) -> Result<()> {
        ic_cdk::call(self.0, "updateCanistergeekInformation", (arg0,)).await
    }
    pub async fn update_emr_registry_principal(
        &self,
        arg0: UpdateEmrRegistryRequest,
    ) -> Result<()> {
        ic_cdk::call(self.0, "update_emr_registry_principal", (arg0,)).await
    }
    pub async fn update_provider_registry_principal(
        &self,
        arg0: UpdateEmrRegistryRequest,
    ) -> Result<()> {
        ic_cdk::call(self.0, "update_provider_registry_principal", (arg0,)).await
    }
}
pub const CANISTER_ID: Principal = Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 3, 1, 1]);
pub const patient_registry: PatientRegistry = PatientRegistry(CANISTER_ID);
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
    pub struct PatientRegistry(pub ic_principal::Principal);
    impl PatientRegistry {
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
            let payload = (arg0,);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "add_authorized_metrics_collector",
                payload,
            )
        }
        pub fn claim_consent(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: ClaimConsentRequest,
        ) -> std::result::Result<ClaimConsentResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(server, f, self.0.clone(), sender, "claim_consent", payload)
        }
        pub fn create_consent(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: CreateConsentRequest,
        ) -> std::result::Result<ClaimConsentRequest, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(server, f, self.0.clone(), sender, "create_consent", payload)
        }
        pub fn emr_list_patient(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: EmrListPatientRequest,
        ) -> std::result::Result<EmrListPatientResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "emr_list_patient",
                payload,
            )
        }
        pub fn emr_list_with_session(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: ClaimConsentResponse,
        ) -> std::result::Result<EmrListPatientResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "emr_list_with_session",
                payload,
            )
        }
        pub fn finish_session(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: ClaimConsentResponse,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(server, f, self.0.clone(), sender, "finish_session", payload)
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
            let payload = (arg0,);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "get_canistergeek_information",
                payload,
            )
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
        pub fn notify_issued(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: IssueRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(server, f, self.0.clone(), sender, "notify_issued", payload)
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
        pub fn read_emr_by_id(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: ReadEmrByIdRequest,
        ) -> std::result::Result<ReadEmrByIdResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(server, f, self.0.clone(), sender, "read_emr_by_id", payload)
        }
        pub fn read_emr_with_session(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: ReadEmrSessionRequest,
        ) -> std::result::Result<ReadEmrByIdResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "read_emr_with_session",
                payload,
            )
        }
        pub fn register_patient(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: RegisterPatientRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "register_patient",
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
            let payload = (arg0,);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "remove_authorized_metrics_collector",
                payload,
            )
        }
        pub fn revoke_consent(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: ClaimConsentRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(server, f, self.0.clone(), sender, "revoke_consent", payload)
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
            let payload = (arg0,);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "update_canistergeek_information",
                payload,
            )
        }
        pub fn update_emr_registry_principal(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: UpdateEmrRegistryRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "update_emr_registry_principal",
                payload,
            )
        }
        pub fn update_provider_registry_principal(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: UpdateEmrRegistryRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0,);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "update_provider_registry_principal",
                payload,
            )
        }
    }
}
