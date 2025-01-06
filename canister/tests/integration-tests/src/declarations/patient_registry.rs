#![allow(warnings, unused_imports)]
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;
#[derive(CandidType, Deserialize)]
pub struct AuthorizedCallerRequest {
    pub caller: Principal,
}
#[derive(CandidType, Deserialize)]
pub enum Relation {
    Parent,
    Sibling,
    Other,
    Child,
    Spouse,
}
#[derive(CandidType, Deserialize)]
pub struct AddGroupMemberRequest {
    pub relation: Relation,
    pub group_id: String,
    pub group_consent_code: String,
}
#[derive(CandidType, Deserialize)]
pub enum Result_ {
    Ok,
    Err(String),
}
#[derive(CandidType, Deserialize)]
pub struct BindAdminRequest {
    pub nik: String,
    pub principal: Principal,
}
#[derive(CandidType, Deserialize)]
pub struct CheckNikRequest {
    pub _type: Option<bool>,
    pub nik: String,
}
#[derive(CandidType, Deserialize)]
pub enum Result1 {
    Ok(bool),
    Err(String),
}
#[derive(CandidType, Deserialize)]
pub struct ClaimConsentRequest {
    pub code: String,
}
#[derive(CandidType, Deserialize)]
pub struct ClaimConsentResponse {
    pub session_id: String,
    pub name: String,
}
#[derive(CandidType, Deserialize)]
pub struct Consent {
    pub nik: String,
    pub group_claimer: Option<Principal>,
    pub session_id: Option<String>,
    pub code: String,
    pub claimed: bool,
    pub session_user: Option<String>,
}
#[derive(CandidType, Deserialize)]
pub struct ConsentListResponse {
    pub consents: Vec<Consent>,
}
#[derive(CandidType, Deserialize)]
pub struct CreateConsentForGroupRequest {
    pub nik: String,
}
#[derive(CandidType, Deserialize)]
pub struct CreateConsentForGroupResponse {
    pub group_consent_code: String,
}
#[derive(CandidType, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
}
#[derive(CandidType, Deserialize)]
pub struct CreateGroupResponse {
    pub group_id: String,
}
#[derive(CandidType, Deserialize)]
pub enum Result2 {
    Ok(CreateGroupResponse),
    Err(String),
}
#[derive(CandidType, Deserialize)]
pub struct EmrListPatientRequest {
    pub page: u8,
    pub limit: u8,
}
#[derive(CandidType, Deserialize)]
pub struct HeaderStatus {
    pub updated_at: u64,
    pub created_at: u64,
}
#[derive(CandidType, Deserialize)]
pub struct EmrHeader {
    pub provider_id: String,
    pub user_id: String,
    pub emr_id: String,
    pub registry_id: Principal,
}
#[derive(CandidType, Deserialize)]
pub struct EmrHeaderWithStatus {
    pub status: HeaderStatus,
    pub hospital_name: String,
    pub header: EmrHeader,
}
#[derive(CandidType, Deserialize)]
pub struct EmrListPatientResponse {
    pub emrs: Vec<EmrHeaderWithStatus>,
}
#[derive(CandidType, Deserialize)]
pub struct EmrListConsentRequest {
    pub session_id: String,
    pub page: u8,
    pub limit: u8,
}
#[derive(CandidType, Deserialize)]
pub struct EmrListConsentResponse {
    pub emr: Vec<EmrHeaderWithStatus>,
    pub username: String,
}
#[derive(CandidType, Deserialize)]
pub struct FinishSessionRequest {
    pub session_id: String,
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
pub struct GetGroupDetailsRequest {
    pub page: u64,
    pub limit: u64,
    pub group_id: String,
}
#[derive(CandidType, Deserialize)]
pub struct GroupDetail {
    pub age: u8,
    pub nik: String,
    pub name: String,
    pub role: Relation,
    pub gender: String,
}
#[derive(CandidType, Deserialize)]
pub struct GetGroupDetailsResponse {
    pub group_details: Vec<GroupDetail>,
    pub total_pages: u64,
    pub leader_name: String,
    pub member_count: u64,
    pub group_name: String,
}
#[derive(CandidType, Deserialize)]
pub enum Result3 {
    Ok(GetGroupDetailsResponse),
    Err(String),
}
#[derive(CandidType, Deserialize)]
pub enum ActivityType {
    Updated,
    Accessed,
    Revoked,
}
#[derive(CandidType, Deserialize)]
pub struct Activity {
    pub activity_type: ActivityType,
    pub provider_id: String,
    pub user_id: String,
    pub timestamp: u64,
}
#[derive(CandidType, Deserialize)]
pub struct LogResponse {
    pub logs: Vec<Activity>,
}
#[derive(CandidType, Deserialize)]
pub enum KycStatus {
    Approved,
    Denied,
    Pending,
}
#[derive(CandidType, Deserialize)]
pub struct V1 {
    pub kyc_date: String,
    pub name: String,
    pub martial_status: String,
    pub place_of_birth: String,
    pub address: String,
    pub gender: String,
    pub kyc_status: KycStatus,
    pub date_of_birth: String,
}
#[derive(CandidType, Deserialize)]
pub enum Patient {
    V1(V1),
}
#[derive(CandidType, Deserialize)]
pub struct GetPatientInfoResponse {
    pub nik: String,
    pub patient: Patient,
}
#[derive(CandidType, Deserialize)]
pub struct PatientWithNik {
    pub nik: String,
    pub info: Patient,
}
#[derive(CandidType, Deserialize)]
pub struct PatientListAdminResponse {
    pub patients: Vec<PatientWithNik>,
}
#[derive(CandidType, Deserialize)]
pub struct Group {
    pub id: String,
    pub members: Vec<String>,
    pub name: String,
    pub leader: String,
    pub member_relations: Vec<(String, Relation)>,
}
#[derive(CandidType, Deserialize)]
pub struct GetUserGroupsResponse {
    pub groups: Vec<Group>,
}
#[derive(CandidType, Deserialize)]
pub struct GrantGroupAccessRequest {
    pub group_id: String,
    pub grantee_nik: String,
}
#[derive(CandidType, Deserialize)]
pub struct IsConsentClaimedResponse {
    pub info: Option<Consent>,
    pub claimed: bool,
}
#[derive(CandidType, Deserialize)]
pub struct LeaveGroupRequest {
    pub group_id: String,
}
#[derive(CandidType, Deserialize)]
pub struct IssueRequest {
    pub header: EmrHeader,
}
#[derive(CandidType, Deserialize)]
pub struct PatientWithNikAndSession {
    pub nik: String,
    pub session_id: String,
    pub info: Patient,
}
#[derive(CandidType, Deserialize)]
pub struct PatientListResponse {
    pub patients: Vec<PatientWithNikAndSession>,
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
pub struct ReadGroupMembersEmrInfoRequest {
    pub provider_id: String,
    pub emr_id: String,
    pub group_id: String,
    pub registry_id: Principal,
    pub member_nik: String,
}
#[derive(CandidType, Deserialize)]
pub enum Result4 {
    Ok(ReadEmrByIdResponse),
    Err(String),
}
#[derive(CandidType, Deserialize)]
pub struct RegisterPatientRequest {
    pub nik: String,
}
#[derive(CandidType, Deserialize)]
pub enum RegisterPatientStatus {
    Error(String),
    Success,
}
#[derive(CandidType, Deserialize)]
pub struct RegisterPatientResponse {
    pub nik: String,
    pub result: RegisterPatientStatus,
}
#[derive(CandidType, Deserialize)]
pub struct RevokeConsentRequest {
    pub codes: Vec<String>,
}
#[derive(CandidType, Deserialize)]
pub struct RevokeGroupAccessRequest {
    pub revokee_nik: String,
    pub group_id: String,
}
#[derive(CandidType, Deserialize)]
pub struct SearchPatientRequest {
    pub _type: Option<String>,
    pub nik: String,
}
#[derive(CandidType, Deserialize)]
pub struct SearchPatientResponse {
    pub patient_info: PatientWithNikAndSession,
}
#[derive(CandidType, Deserialize)]
pub struct SearchPatientAdminResponse {
    pub patient_info: PatientWithNik,
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
#[derive(CandidType, Deserialize)]
pub struct UpdateKycStatusRequest {
    pub nik: String,
    pub kyc_status: KycStatus,
}
#[derive(CandidType, Deserialize)]
pub struct UpdateKycStatusResponse {
    pub patient: Patient,
}
#[derive(CandidType, Deserialize)]
pub struct UpdatePatientInfoRequest {
    pub info: V1,
}
#[derive(CandidType, Deserialize)]
pub struct ViewGroupMemberEmrInformationRequest {
    pub page: u64,
    pub limit: u64,
    pub group_id: String,
    pub member_nik: String,
}
#[derive(CandidType, Deserialize)]
pub enum Result5 {
    Ok(EmrListPatientResponse),
    Err(String),
}
pub struct PatientRegistry(pub Principal);
impl PatientRegistry {
    pub async fn add_authorized_metrics_collector(
        &self,
        arg0: AuthorizedCallerRequest,
    ) -> Result<()> {
        ic_cdk::call(self.0, "add_authorized_metrics_collector", (arg0,)).await
    }
    pub async fn add_group_member(&self, arg0: AddGroupMemberRequest) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "add_group_member", (arg0,)).await
    }
    pub async fn bind_admin(&self, arg0: BindAdminRequest) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "bind_admin", (arg0,)).await
    }
    pub async fn bind_admin_principal_only(&self, arg0: Principal) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "bind_admin_principal_only", (arg0,)).await
    }
    pub async fn check_admin(&self, arg0: Principal) -> Result<(bool,)> {
        ic_cdk::call(self.0, "check_admin", (arg0,)).await
    }
    pub async fn check_nik(&self, arg0: CheckNikRequest) -> Result<(Result1,)> {
        ic_cdk::call(self.0, "check_nik", (arg0,)).await
    }
    pub async fn claim_consent(
        &self,
        arg0: ClaimConsentRequest,
    ) -> Result<(ClaimConsentResponse,)> {
        ic_cdk::call(self.0, "claim_consent", (arg0,)).await
    }
    pub async fn consent_list(&self) -> Result<(ConsentListResponse,)> {
        ic_cdk::call(self.0, "consent_list", ()).await
    }
    pub async fn create_consent(&self) -> Result<(ClaimConsentRequest,)> {
        ic_cdk::call(self.0, "create_consent", ()).await
    }
    pub async fn create_consent_for_group(
        &self,
        arg0: CreateConsentForGroupRequest,
    ) -> Result<(CreateConsentForGroupResponse,)> {
        ic_cdk::call(self.0, "create_consent_for_group", (arg0,)).await
    }
    pub async fn create_group(&self, arg0: CreateGroupRequest) -> Result<(Result2,)> {
        ic_cdk::call(self.0, "create_group", (arg0,)).await
    }
    pub async fn emr_list_patient(
        &self,
        arg0: EmrListPatientRequest,
    ) -> Result<(EmrListPatientResponse,)> {
        ic_cdk::call(self.0, "emr_list_patient", (arg0,)).await
    }
    pub async fn emr_list_with_session(
        &self,
        arg0: EmrListConsentRequest,
    ) -> Result<(EmrListConsentResponse,)> {
        ic_cdk::call(self.0, "emr_list_with_session", (arg0,)).await
    }
    pub async fn finish_session(&self, arg0: FinishSessionRequest) -> Result<()> {
        ic_cdk::call(self.0, "finish_session", (arg0,)).await
    }
    pub async fn get_canistergeek_information(
        &self,
        arg0: GetInformationRequest,
    ) -> Result<(GetInformationResponse,)> {
        ic_cdk::call(self.0, "getCanistergeekInformation", (arg0,)).await
    }
    pub async fn get_group_details(&self, arg0: GetGroupDetailsRequest) -> Result<(Result3,)> {
        ic_cdk::call(self.0, "get_group_details", (arg0,)).await
    }
    pub async fn get_group_details_admin(
        &self,
        arg0: GetGroupDetailsRequest,
    ) -> Result<(Result3,)> {
        ic_cdk::call(self.0, "get_group_details_admin", (arg0,)).await
    }
    pub async fn get_group_details_async_no_pagination(
        &self,
        arg0: CreateGroupResponse,
    ) -> Result<(Result3,)> {
        ic_cdk::call(self.0, "get_group_details_async_no_pagination", (arg0,)).await
    }
    pub async fn get_logs(&self) -> Result<(LogResponse,)> {
        ic_cdk::call(self.0, "get_logs", ()).await
    }
    pub async fn get_patient_info(&self) -> Result<(GetPatientInfoResponse,)> {
        ic_cdk::call(self.0, "get_patient_info", ()).await
    }
    pub async fn get_patient_info_with_consent(
        &self,
        arg0: FinishSessionRequest,
    ) -> Result<(GetPatientInfoResponse,)> {
        ic_cdk::call(self.0, "get_patient_info_with_consent", (arg0,)).await
    }
    pub async fn get_patient_list_admin(&self) -> Result<(PatientListAdminResponse,)> {
        ic_cdk::call(self.0, "get_patient_list_admin", ()).await
    }
    pub async fn get_trusted_origins(&self) -> Result<(Vec<String>,)> {
        ic_cdk::call(self.0, "get_trusted_origins", ()).await
    }
    pub async fn get_user_groups(&self) -> Result<(GetUserGroupsResponse,)> {
        ic_cdk::call(self.0, "get_user_groups", ()).await
    }
    pub async fn grant_group_access(&self, arg0: GrantGroupAccessRequest) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "grant_group_access", (arg0,)).await
    }
    pub async fn is_consent_claimed(
        &self,
        arg0: ClaimConsentRequest,
    ) -> Result<(IsConsentClaimedResponse,)> {
        ic_cdk::call(self.0, "is_consent_claimed", (arg0,)).await
    }
    pub async fn leave_group(&self, arg0: LeaveGroupRequest) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "leave_group", (arg0,)).await
    }
    pub async fn metrics(&self) -> Result<(String,)> {
        ic_cdk::call(self.0, "metrics", ()).await
    }
    pub async fn notify_issued(&self, arg0: IssueRequest) -> Result<()> {
        ic_cdk::call(self.0, "notify_issued", (arg0,)).await
    }
    pub async fn notify_updated(&self, arg0: IssueRequest) -> Result<()> {
        ic_cdk::call(self.0, "notify_updated", (arg0,)).await
    }
    pub async fn patient_list(&self) -> Result<(PatientListResponse,)> {
        ic_cdk::call(self.0, "patient_list", ()).await
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
    pub async fn read_group_members_emr_info(
        &self,
        arg0: ReadGroupMembersEmrInfoRequest,
    ) -> Result<(Result4,)> {
        ic_cdk::call(self.0, "read_group_members_emr_info", (arg0,)).await
    }
    pub async fn register_patient(
        &self,
        arg0: RegisterPatientRequest,
    ) -> Result<(RegisterPatientResponse,)> {
        ic_cdk::call(self.0, "register_patient", (arg0,)).await
    }
    pub async fn remove_authorized_metrics_collector(
        &self,
        arg0: AuthorizedCallerRequest,
    ) -> Result<()> {
        ic_cdk::call(self.0, "remove_authorized_metrics_collector", (arg0,)).await
    }
    pub async fn revoke_consent(&self, arg0: RevokeConsentRequest) -> Result<()> {
        ic_cdk::call(self.0, "revoke_consent", (arg0,)).await
    }
    pub async fn revoke_group_access(&self, arg0: RevokeGroupAccessRequest) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "revoke_group_access", (arg0,)).await
    }
    pub async fn search_patient(
        &self,
        arg0: SearchPatientRequest,
    ) -> Result<(SearchPatientResponse,)> {
        ic_cdk::call(self.0, "search_patient", (arg0,)).await
    }
    pub async fn search_patient_admin(
        &self,
        arg0: SearchPatientRequest,
    ) -> Result<(SearchPatientAdminResponse,)> {
        ic_cdk::call(self.0, "search_patient_admin", (arg0,)).await
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
    pub async fn update_kyc_status(
        &self,
        arg0: UpdateKycStatusRequest,
    ) -> Result<(UpdateKycStatusResponse,)> {
        ic_cdk::call(self.0, "update_kyc_status", (arg0,)).await
    }
    pub async fn update_patient_info(&self, arg0: UpdatePatientInfoRequest) -> Result<()> {
        ic_cdk::call(self.0, "update_patient_info", (arg0,)).await
    }
    pub async fn update_provider_registry_principal(
        &self,
        arg0: UpdateEmrRegistryRequest,
    ) -> Result<()> {
        ic_cdk::call(self.0, "update_provider_registry_principal", (arg0,)).await
    }
    pub async fn view_group_member_emr_information(
        &self,
        arg0: ViewGroupMemberEmrInformationRequest,
    ) -> Result<(Result5,)> {
        ic_cdk::call(self.0, "view_group_member_emr_information", (arg0,)).await
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
        pub fn add_group_member(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: AddGroupMemberRequest,
        ) -> std::result::Result<Result_, pocket_ic::UserError> {
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
                "add_group_member",
                payload,
            )
        }
        pub fn bind_admin(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: BindAdminRequest,
        ) -> std::result::Result<Result_, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "bind_admin", payload)
        }
        pub fn bind_admin_principal_only(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: Principal,
        ) -> std::result::Result<Result_, pocket_ic::UserError> {
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
                "bind_admin_principal_only",
                payload,
            )
        }
        pub fn check_admin(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: Principal,
        ) -> std::result::Result<bool, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "check_admin", payload)
        }
        pub fn check_nik(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: CheckNikRequest,
        ) -> std::result::Result<Result1, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "check_nik", payload)
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
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "claim_consent", payload)
        }
        pub fn consent_list(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
        ) -> std::result::Result<ConsentListResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = ();
            call_pocket_ic(server, f, self.0.clone(), sender, "consent_list", payload)
        }
        pub fn create_consent(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
        ) -> std::result::Result<ClaimConsentRequest, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = ();
            call_pocket_ic(server, f, self.0.clone(), sender, "create_consent", payload)
        }
        pub fn create_consent_for_group(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: CreateConsentForGroupRequest,
        ) -> std::result::Result<CreateConsentForGroupResponse, pocket_ic::UserError> {
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
                "create_consent_for_group",
                payload,
            )
        }
        pub fn create_group(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: CreateGroupRequest,
        ) -> std::result::Result<Result2, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "create_group", payload)
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
            let payload = (arg0);
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
            arg0: EmrListConsentRequest,
        ) -> std::result::Result<EmrListConsentResponse, pocket_ic::UserError> {
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
                "emr_list_with_session",
                payload,
            )
        }
        pub fn finish_session(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: FinishSessionRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
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
        pub fn get_group_details(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: GetGroupDetailsRequest,
        ) -> std::result::Result<Result3, pocket_ic::UserError> {
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
                "get_group_details",
                payload,
            )
        }
        pub fn get_group_details_admin(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: GetGroupDetailsRequest,
        ) -> std::result::Result<Result3, pocket_ic::UserError> {
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
                "get_group_details_admin",
                payload,
            )
        }
        pub fn get_group_details_async_no_pagination(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: CreateGroupResponse,
        ) -> std::result::Result<Result3, pocket_ic::UserError> {
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
                "get_group_details_async_no_pagination",
                payload,
            )
        }
        pub fn get_logs(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
        ) -> std::result::Result<LogResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = ();
            call_pocket_ic(server, f, self.0.clone(), sender, "get_logs", payload)
        }
        pub fn get_patient_info(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
        ) -> std::result::Result<GetPatientInfoResponse, pocket_ic::UserError> {
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
                "get_patient_info",
                payload,
            )
        }
        pub fn get_patient_info_with_consent(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: FinishSessionRequest,
        ) -> std::result::Result<GetPatientInfoResponse, pocket_ic::UserError> {
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
                "get_patient_info_with_consent",
                payload,
            )
        }
        pub fn get_patient_list_admin(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
        ) -> std::result::Result<PatientListAdminResponse, pocket_ic::UserError> {
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
                "get_patient_list_admin",
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
        pub fn get_user_groups(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
        ) -> std::result::Result<GetUserGroupsResponse, pocket_ic::UserError> {
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
                "get_user_groups",
                payload,
            )
        }
        pub fn grant_group_access(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: GrantGroupAccessRequest,
        ) -> std::result::Result<Result_, pocket_ic::UserError> {
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
                "grant_group_access",
                payload,
            )
        }
        pub fn is_consent_claimed(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: ClaimConsentRequest,
        ) -> std::result::Result<IsConsentClaimedResponse, pocket_ic::UserError> {
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
                "is_consent_claimed",
                payload,
            )
        }
        pub fn leave_group(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: LeaveGroupRequest,
        ) -> std::result::Result<Result_, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "leave_group", payload)
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
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "notify_issued", payload)
        }
        pub fn notify_updated(
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
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "notify_updated", payload)
        }
        pub fn patient_list(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
        ) -> std::result::Result<PatientListResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = ();
            call_pocket_ic(server, f, self.0.clone(), sender, "patient_list", payload)
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
            let payload = (arg0);
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
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "read_emr_with_session",
                payload,
            )
        }
        pub fn read_group_members_emr_info(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: ReadGroupMembersEmrInfoRequest,
        ) -> std::result::Result<Result4, pocket_ic::UserError> {
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
                "read_group_members_emr_info",
                payload,
            )
        }
        pub fn register_patient(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: RegisterPatientRequest,
        ) -> std::result::Result<RegisterPatientResponse, pocket_ic::UserError> {
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
        pub fn revoke_consent(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: RevokeConsentRequest,
        ) -> std::result::Result<(), pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "revoke_consent", payload)
        }
        pub fn revoke_group_access(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: RevokeGroupAccessRequest,
        ) -> std::result::Result<Result_, pocket_ic::UserError> {
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
                "revoke_group_access",
                payload,
            )
        }
        pub fn search_patient(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: SearchPatientRequest,
        ) -> std::result::Result<SearchPatientResponse, pocket_ic::UserError> {
            let f = match call_type {
                Call::Query => pocket_ic::PocketIc::query_call,
                Call::Update => pocket_ic::PocketIc::update_call,
            };
            let payload = (arg0);
            call_pocket_ic(server, f, self.0.clone(), sender, "search_patient", payload)
        }
        pub fn search_patient_admin(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: SearchPatientRequest,
        ) -> std::result::Result<SearchPatientAdminResponse, pocket_ic::UserError> {
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
                "search_patient_admin",
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
        pub fn update_kyc_status(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: UpdateKycStatusRequest,
        ) -> std::result::Result<UpdateKycStatusResponse, pocket_ic::UserError> {
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
                "update_kyc_status",
                payload,
            )
        }
        pub fn update_patient_info(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: UpdatePatientInfoRequest,
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
                "update_patient_info",
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
            let payload = (arg0);
            call_pocket_ic(
                server,
                f,
                self.0.clone(),
                sender,
                "update_provider_registry_principal",
                payload,
            )
        }
        pub fn view_group_member_emr_information(
            &self,
            server: &pocket_ic::PocketIc,
            sender: ic_principal::Principal,
            call_type: Call,
            arg0: ViewGroupMemberEmrInformationRequest,
        ) -> std::result::Result<Result5, pocket_ic::UserError> {
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
                "view_group_member_emr_information",
                payload,
            )
        }
    }
}
