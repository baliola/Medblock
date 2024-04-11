import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AuthorizedCallerRequest { 'caller' : Principal }
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
export interface ClaimConsentRequest { 'code' : string }
export interface ClaimConsentResponse { 'session_id' : string }
export type CollectMetricsRequestType = { 'force' : null } |
  { 'normal' : null };
export interface CreateConsentRequest { 'allowed' : Array<EmrHeader> }
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
export interface EmrListPatientRequest { 'page' : number, 'limit' : number }
export interface EmrListPatientResponse { 'emrs' : Array<EmrHeader> }
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
export interface HourlyMetricsData {
  'updateCalls' : BigUint64Array | bigint[],
  'canisterHeapMemorySize' : BigUint64Array | bigint[],
  'canisterCycles' : BigUint64Array | bigint[],
  'canisterMemorySize' : BigUint64Array | bigint[],
  'timeMillis' : bigint,
}
export interface IssueRequest { 'header' : EmrHeader }
export interface LogMessageData { 'timeNanos' : bigint, 'message' : string }
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
export interface RegisterPatientRequest { 'nik' : string }
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
export interface _SERVICE {
  'add_authorized_metrics_collector' : ActorMethod<
    [AuthorizedCallerRequest],
    undefined
  >,
  'claim_consent' : ActorMethod<[ClaimConsentRequest], ClaimConsentResponse>,
  'create_consent' : ActorMethod<[CreateConsentRequest], ClaimConsentRequest>,
  'emr_list_patient' : ActorMethod<
    [EmrListPatientRequest],
    EmrListPatientResponse
  >,
  'emr_list_with_session' : ActorMethod<
    [ClaimConsentResponse],
    EmrListPatientResponse
  >,
  'finish_session' : ActorMethod<[ClaimConsentResponse], undefined>,
  'getCanistergeekInformation' : ActorMethod<
    [GetInformationRequest],
    GetInformationResponse
  >,
  'metrics' : ActorMethod<[], string>,
  'notify_issued' : ActorMethod<[IssueRequest], undefined>,
  'ping' : ActorMethod<[], PingResult>,
  'read_emr_by_id' : ActorMethod<[ReadEmrByIdRequest], ReadEmrByIdResponse>,
  'read_emr_with_session' : ActorMethod<
    [ReadEmrSessionRequest],
    ReadEmrByIdResponse
  >,
  'register_patient' : ActorMethod<[RegisterPatientRequest], undefined>,
  'remove_authorized_metrics_collector' : ActorMethod<
    [AuthorizedCallerRequest],
    undefined
  >,
  'revoke_consent' : ActorMethod<[ClaimConsentRequest], undefined>,
  'updateCanistergeekInformation' : ActorMethod<
    [UpdateInformationRequest],
    undefined
  >,
  'update_emr_registry_principal' : ActorMethod<
    [UpdateEmrRegistryRequest],
    undefined
  >,
  'update_provider_registry_principal' : ActorMethod<
    [UpdateEmrRegistryRequest],
    undefined
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
