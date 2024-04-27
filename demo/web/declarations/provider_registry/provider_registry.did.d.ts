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
export type CollectMetricsRequestType = { 'force' : null } |
  { 'normal' : null };
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
export interface EmrListProviderRequest { 'page' : bigint, 'limit' : number }
export interface EmrListProviderResponse { 'ids' : Array<string> }
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
export interface GetProviderBatchRequest { 'ids' : Array<string> }
export interface GetProviderBatchResponse { 'providers' : Array<Provider> }
export interface Header {
  'provider_id' : string,
  'user_id' : string,
  'emr_id' : string,
  'registry_id' : Principal,
}
export interface HourlyMetricsData {
  'updateCalls' : BigUint64Array | bigint[],
  'canisterHeapMemorySize' : BigUint64Array | bigint[],
  'canisterCycles' : BigUint64Array | bigint[],
  'canisterMemorySize' : BigUint64Array | bigint[],
  'timeMillis' : bigint,
}
export interface IssueEmrRequest {
  'emr' : Array<EmrFragment>,
  'user_id' : string,
}
export interface IssueEmrResponse { 'emr_header' : Header }
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
export interface PingResult {
  'patient_registry_status' : boolean,
  'emr_registry_status' : boolean,
}
export type Provider = { 'V1' : V1 };
export interface ProviderInfoRequest { 'provider' : Array<Principal> }
export interface ProviderInfoResponse { 'providers' : Array<Provider> }
export interface RegisternewProviderRequest {
  'provider_principal' : Principal,
  'display_name' : string,
  'address' : string,
}
export type Status = { 'Active' : null } |
  { 'Suspended' : null };
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
export interface SuspendRequest { 'principal' : Principal }
export interface UpdateEmrRequest {
  'fields' : Array<EmrFragment>,
  'header' : EmrHeader,
}
export interface UpdateInformationRequest {
  'metrics' : [] | [CollectMetricsRequestType],
}
export interface V1 {
  'updated_at' : bigint,
  'internal_id' : string,
  'display_name' : string,
  'session' : bigint,
  'address' : string,
  'registered_at' : bigint,
  'activation_status' : Status,
}
export interface _SERVICE {
  'add_authorized_metrics_collector' : ActorMethod<
    [AuthorizedCallerRequest],
    undefined
  >,
  'emr_list_provider' : ActorMethod<
    [EmrListProviderRequest],
    EmrListProviderResponse
  >,
  'getCanistergeekInformation' : ActorMethod<
    [GetInformationRequest],
    GetInformationResponse
  >,
  'get_provider_batch' : ActorMethod<
    [GetProviderBatchRequest],
    GetProviderBatchResponse
  >,
  'get_provider_info_with_principal' : ActorMethod<
    [ProviderInfoRequest],
    ProviderInfoResponse
  >,
  'get_trusted_origins' : ActorMethod<[], Array<string>>,
  'issue_emr' : ActorMethod<[IssueEmrRequest], IssueEmrResponse>,
  'metrics' : ActorMethod<[], string>,
  'ping' : ActorMethod<[], PingResult>,
  'register_new_provider' : ActorMethod<[RegisternewProviderRequest], {}>,
  'remove_authorized_metrics_collector' : ActorMethod<
    [AuthorizedCallerRequest],
    undefined
  >,
  'suspend_provider' : ActorMethod<[SuspendRequest], undefined>,
  'unsuspend_provider' : ActorMethod<[SuspendRequest], undefined>,
  'updateCanistergeekInformation' : ActorMethod<
    [UpdateInformationRequest],
    undefined
  >,
  'update_emr' : ActorMethod<[UpdateEmrRequest], {}>,
  'update_emr_registry_principal' : ActorMethod<[SuspendRequest], undefined>,
  'update_patient_registry_principal' : ActorMethod<
    [SuspendRequest],
    undefined
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
