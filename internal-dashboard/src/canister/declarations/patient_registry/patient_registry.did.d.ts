import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Activity {
  'activity_type' : ActivityType,
  'provider_id' : string,
  'user_id' : string,
  'timestamp' : bigint,
}
export type ActivityType = { 'Updated' : null } |
  { 'Accessed' : null } |
  { 'Revoked' : null };
export interface AddGroupMemberRequest {
  'relation' : Relation,
  'group_id' : string,
  'group_consent_code' : string,
}
export interface AuthorizedCallerRequest { 'caller' : Principal }
export interface BindAdminRequest { 'nik' : string, 'principal' : Principal }
export type CanisterLogFeature = { 'filterMessageByContains' : null } |
  { 'filterMessageByRegex' : null };
export interface CanisterLogMessages {
  'data' : Array<LogMessageData>,
  'lastAnalyzedMessageTimeNanos' : [] | [bigint],
}
export interface CanisterLogMessagesInfo {
  'features' : Array<[] | [CanisterLogFeature]>,
  'lastTimeNanos' : [] | [bigint],
  'count' : number,
  'firstTimeNanos' : [] | [bigint],
}
export type CanisterLogRequest = { 'getMessagesInfo' : null } |
  { 'getMessages' : GetLogMessagesParameters } |
  { 'getLatestMessages' : GetLatestLogMessagesParameters };
export type CanisterLogResponse = { 'messagesInfo' : CanisterLogMessagesInfo } |
  { 'messages' : CanisterLogMessages };
export interface CanisterMetrics { 'data' : CanisterMetricsData }
export type CanisterMetricsData = { 'hourly' : Array<HourlyMetricsData> } |
  { 'daily' : Array<DailyMetricsData> };
export interface CheckNikRequest { '_type' : [] | [boolean], 'nik' : string }
export interface ClaimConsentRequest { 'code' : string }
export interface ClaimConsentResponse { 'session_id' : string, 'name' : string }
export type CollectMetricsRequestType = { 'force' : null } |
  { 'normal' : null };
export interface Consent {
  'nik' : string,
  'group_claimer' : [] | [Principal],
  'session_id' : [] | [string],
  'code' : string,
  'claimed' : boolean,
  'session_user' : [] | [string],
}
export interface ConsentListResponse { 'consents' : Array<Consent> }
export interface CreateConsentForGroupRequest { 'nik' : string }
export interface CreateConsentForGroupResponse { 'group_consent_code' : string }
export interface CreateGroupRequest { 'name' : string }
export interface CreateGroupResponse { 'group_id' : string }
export interface DailyMetricsData {
  'updateCalls' : bigint,
  'canisterHeapMemorySize' : NumericEntity,
  'canisterCycles' : NumericEntity,
  'canisterMemorySize' : NumericEntity,
  'timeMillis' : bigint,
}
export interface EmrFragment { 'key' : string, 'value' : string }
export interface EmrHeader {
  'provider_id' : string,
  'user_id' : string,
  'emr_id' : string,
  'registry_id' : Principal,
}
export interface EmrHeaderWithBody {
  'body' : Array<EmrFragment>,
  'header' : EmrHeader,
}
export interface EmrHeaderWithStatus {
  'status' : HeaderStatus,
  'hospital_name' : string,
  'header' : EmrHeader,
}
export interface EmrListConsentRequest {
  'session_id' : string,
  'page' : number,
  'limit' : number,
}
export interface EmrListConsentResponse {
  'emr' : Array<EmrHeaderWithStatus>,
  'username' : string,
}
export interface EmrListPatientRequest { 'page' : number, 'limit' : number }
export interface EmrListPatientResponse { 'emrs' : Array<EmrHeaderWithStatus> }
export interface FinishSessionRequest { 'session_id' : string }
export interface GetGroupDetailsRequest {
  'page' : bigint,
  'limit' : bigint,
  'group_id' : string,
}
export interface GetGroupDetailsResponse {
  'group_details' : Array<GroupDetail>,
  'total_pages' : bigint,
  'leader_name' : string,
  'member_count' : bigint,
  'group_name' : string,
}
export interface GetInformationRequest {
  'status' : [] | [StatusRequest],
  'metrics' : [] | [MetricsRequest],
  'logs' : [] | [CanisterLogRequest],
  'version' : boolean,
}
export interface GetInformationResponse {
  'status' : [] | [StatusResponse],
  'metrics' : [] | [MetricsResponse],
  'logs' : [] | [CanisterLogResponse],
  'version' : [] | [bigint],
}
export interface GetLatestLogMessagesParameters {
  'upToTimeNanos' : [] | [bigint],
  'count' : number,
  'filter' : [] | [GetLogMessagesFilter],
}
export interface GetLogMessagesFilter {
  'analyzeCount' : number,
  'messageRegex' : [] | [string],
  'messageContains' : [] | [string],
}
export interface GetLogMessagesParameters {
  'count' : number,
  'filter' : [] | [GetLogMessagesFilter],
  'fromTimeNanos' : [] | [bigint],
}
export interface GetMetricsParameters {
  'dateToMillis' : bigint,
  'granularity' : MetricsGranularity,
  'dateFromMillis' : bigint,
}
export interface GetPatientInfoResponse { 'nik' : string, 'patient' : Patient }
export interface GetUserGroupsResponse { 'groups' : Array<Group> }
export interface GrantGroupAccessRequest {
  'group_id' : string,
  'grantee_nik' : string,
}
export interface Group {
  'id' : string,
  'members' : Array<string>,
  'name' : string,
  'leader' : string,
  'member_relations' : Array<[string, Relation]>,
}
export interface GroupDetail {
  'age' : number,
  'nik' : string,
  'name' : string,
  'role' : Relation,
  'gender' : string,
}
export interface HeaderStatus { 'updated_at' : bigint, 'created_at' : bigint }
export interface HourlyMetricsData {
  'updateCalls' : BigUint64Array | bigint[],
  'canisterHeapMemorySize' : BigUint64Array | bigint[],
  'canisterCycles' : BigUint64Array | bigint[],
  'canisterMemorySize' : BigUint64Array | bigint[],
  'timeMillis' : bigint,
}
export interface IsConsentClaimedResponse {
  'info' : [] | [Consent],
  'claimed' : boolean,
}
export interface IssueRequest { 'header' : EmrHeader }
export type KycStatus = { 'Approved' : null } |
  { 'Denied' : null } |
  { 'Pending' : null };
export interface LeaveGroupRequest { 'group_id' : string }
export interface LogMessageData { 'timeNanos' : bigint, 'message' : string }
export interface LogResponse { 'logs' : Array<Activity> }
export type MetricsGranularity = { 'hourly' : null } |
  { 'daily' : null };
export interface MetricsRequest { 'parameters' : GetMetricsParameters }
export interface MetricsResponse { 'metrics' : [] | [CanisterMetrics] }
export interface NumericEntity {
  'avg' : bigint,
  'max' : bigint,
  'min' : bigint,
  'first' : bigint,
  'last' : bigint,
}
export type Patient = { 'V1' : V1 };
export interface PatientListAdminResponse { 'patients' : Array<PatientWithNik> }
export interface PatientListResponse {
  'patients' : Array<PatientWithNikAndSession>,
}
export interface PatientWithNik { 'nik' : string, 'info' : Patient }
export interface PatientWithNikAndSession {
  'nik' : string,
  'session_id' : string,
  'info' : Patient,
}
export interface PingResult { 'emr_registry_status' : boolean }
export interface ReadEmrByIdRequest {
  'provider_id' : string,
  'emr_id' : string,
  'registry_id' : Principal,
}
export interface ReadEmrByIdResponse { 'emr' : EmrHeaderWithBody }
export interface ReadEmrSessionRequest {
  'session_id' : string,
  'args' : ReadEmrByIdRequest,
}
export interface ReadGroupMembersEmrInfoRequest {
  'provider_id' : string,
  'emr_id' : string,
  'group_id' : string,
  'registry_id' : Principal,
  'member_nik' : string,
}
export interface RegisterPatientRequest { 'nik' : string }
export interface RegisterPatientResponse {
  'nik' : string,
  'result' : RegisterPatientStatus,
}
export type RegisterPatientStatus = { 'Error' : string } |
  { 'Success' : null };
export type Relation = { 'Parent' : null } |
  { 'Sibling' : null } |
  { 'Other' : null } |
  { 'Child' : null } |
  { 'Spouse' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : CreateGroupResponse } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : GetGroupDetailsResponse } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : ReadEmrByIdResponse } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : EmrListPatientResponse } |
  { 'Err' : string };
export interface RevokeConsentRequest { 'codes' : Array<string> }
export interface RevokeGroupAccessRequest { 'grantee_nik' : string }
export interface SearchPatientAdminResponse { 'patient_info' : PatientWithNik }
export interface SearchPatientRequest {
  '_type' : [] | [string],
  'nik' : string,
}
export interface SearchPatientResponse {
  'patient_info' : PatientWithNikAndSession,
}
export interface StatusRequest {
  'memory_size' : boolean,
  'cycles' : boolean,
  'heap_memory_size' : boolean,
}
export interface StatusResponse {
  'memory_size' : [] | [bigint],
  'cycles' : [] | [bigint],
  'heap_memory_size' : [] | [bigint],
}
export interface UpdateEmrRegistryRequest { 'principal' : Principal }
export interface UpdateInformationRequest {
  'metrics' : [] | [CollectMetricsRequestType],
}
export interface UpdateKycStatusRequest {
  'nik' : string,
  'kyc_status' : KycStatus,
}
export interface UpdateKycStatusResponse { 'patient' : Patient }
export interface UpdatePatientInfoRequest { 'info' : V1 }
export interface V1 {
  'kyc_date' : string,
  'name' : string,
  'martial_status' : string,
  'place_of_birth' : string,
  'address' : string,
  'gender' : string,
  'kyc_status' : KycStatus,
  'date_of_birth' : string,
}
export interface ViewGroupMemberEmrInformationRequest {
  'page' : bigint,
  'limit' : bigint,
  'group_id' : string,
  'member_nik' : string,
}
export interface _SERVICE {
  'add_authorized_metrics_collector' : ActorMethod<
    [AuthorizedCallerRequest],
    undefined
  >,
  'add_group_member' : ActorMethod<[AddGroupMemberRequest], Result>,
  'bind_admin' : ActorMethod<[BindAdminRequest], Result>,
  'bind_admin_principal_only' : ActorMethod<[Principal], Result>,
  'check_admin' : ActorMethod<[Principal], boolean>,
  'check_nik' : ActorMethod<[CheckNikRequest], Result_1>,
  'claim_consent' : ActorMethod<[ClaimConsentRequest], ClaimConsentResponse>,
  'consent_list' : ActorMethod<[], ConsentListResponse>,
  'create_consent' : ActorMethod<[], ClaimConsentRequest>,
  'create_consent_for_group' : ActorMethod<
    [CreateConsentForGroupRequest],
    CreateConsentForGroupResponse
  >,
  'create_group' : ActorMethod<[CreateGroupRequest], Result_2>,
  'emr_list_patient' : ActorMethod<
    [EmrListPatientRequest],
    EmrListPatientResponse
  >,
  'emr_list_with_session' : ActorMethod<
    [EmrListConsentRequest],
    EmrListConsentResponse
  >,
  'finish_session' : ActorMethod<[FinishSessionRequest], undefined>,
  'getCanistergeekInformation' : ActorMethod<
    [GetInformationRequest],
    GetInformationResponse
  >,
  'get_group_details' : ActorMethod<[GetGroupDetailsRequest], Result_3>,
  'get_group_details_admin' : ActorMethod<[GetGroupDetailsRequest], Result_3>,
  'get_group_details_async_no_pagination' : ActorMethod<
    [CreateGroupResponse],
    Result_3
  >,
  'get_logs' : ActorMethod<[], LogResponse>,
  'get_patient_info' : ActorMethod<[], GetPatientInfoResponse>,
  'get_patient_info_with_consent' : ActorMethod<
    [FinishSessionRequest],
    GetPatientInfoResponse
  >,
  'get_patient_list_admin' : ActorMethod<[], PatientListAdminResponse>,
  'get_trusted_origins' : ActorMethod<[], Array<string>>,
  'get_user_groups' : ActorMethod<[], GetUserGroupsResponse>,
  'grant_group_access' : ActorMethod<[GrantGroupAccessRequest], Result>,
  'is_consent_claimed' : ActorMethod<
    [ClaimConsentRequest],
    IsConsentClaimedResponse
  >,
  'leave_group' : ActorMethod<[LeaveGroupRequest], Result>,
  'metrics' : ActorMethod<[], string>,
  'notify_issued' : ActorMethod<[IssueRequest], undefined>,
  'notify_updated' : ActorMethod<[IssueRequest], undefined>,
  'patient_list' : ActorMethod<[], PatientListResponse>,
  'ping' : ActorMethod<[], PingResult>,
  'read_emr_by_id' : ActorMethod<[ReadEmrByIdRequest], ReadEmrByIdResponse>,
  'read_emr_with_session' : ActorMethod<
    [ReadEmrSessionRequest],
    ReadEmrByIdResponse
  >,
  'read_group_members_emr_info' : ActorMethod<
    [ReadGroupMembersEmrInfoRequest],
    Result_4
  >,
  'register_patient' : ActorMethod<
    [RegisterPatientRequest],
    RegisterPatientResponse
  >,
  'remove_authorized_metrics_collector' : ActorMethod<
    [AuthorizedCallerRequest],
    undefined
  >,
  'revoke_consent' : ActorMethod<[RevokeConsentRequest], undefined>,
  'revoke_group_access' : ActorMethod<[RevokeGroupAccessRequest], Result>,
  'search_patient' : ActorMethod<[SearchPatientRequest], SearchPatientResponse>,
  'search_patient_admin' : ActorMethod<
    [SearchPatientRequest],
    SearchPatientAdminResponse
  >,
  'updateCanistergeekInformation' : ActorMethod<
    [UpdateInformationRequest],
    undefined
  >,
  'update_emr_registry_principal' : ActorMethod<
    [UpdateEmrRegistryRequest],
    undefined
  >,
  'update_kyc_status' : ActorMethod<
    [UpdateKycStatusRequest],
    UpdateKycStatusResponse
  >,
  'update_patient_info' : ActorMethod<[UpdatePatientInfoRequest], undefined>,
  'update_provider_registry_principal' : ActorMethod<
    [UpdateEmrRegistryRequest],
    undefined
  >,
  'view_group_member_emr_information' : ActorMethod<
    [ViewGroupMemberEmrInformationRequest],
    Result_5
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
