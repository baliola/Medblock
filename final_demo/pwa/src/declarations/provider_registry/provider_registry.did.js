export const idlFactory = ({ IDL }) => {
  const AuthorizedCallerRequest = IDL.Record({ 'caller' : IDL.Principal });
  const EmrListProviderRequest = IDL.Record({
    'page' : IDL.Nat64,
    'limit' : IDL.Nat8,
  });
  const EmrListProviderResponse = IDL.Record({ 'ids' : IDL.Vec(IDL.Text) });
  const StatusRequest = IDL.Record({
    'memory_size' : IDL.Bool,
    'cycles' : IDL.Bool,
    'heap_memory_size' : IDL.Bool,
  });
  const MetricsGranularity = IDL.Variant({
    'hourly' : IDL.Null,
    'daily' : IDL.Null,
  });
  const GetMetricsParameters = IDL.Record({
    'dateToMillis' : IDL.Nat,
    'granularity' : MetricsGranularity,
    'dateFromMillis' : IDL.Nat,
  });
  const MetricsRequest = IDL.Record({ 'parameters' : GetMetricsParameters });
  const GetLogMessagesFilter = IDL.Record({
    'analyzeCount' : IDL.Nat32,
    'messageRegex' : IDL.Opt(IDL.Text),
    'messageContains' : IDL.Opt(IDL.Text),
  });
  const GetLogMessagesParameters = IDL.Record({
    'count' : IDL.Nat32,
    'filter' : IDL.Opt(GetLogMessagesFilter),
    'fromTimeNanos' : IDL.Opt(IDL.Nat64),
  });
  const GetLatestLogMessagesParameters = IDL.Record({
    'upToTimeNanos' : IDL.Opt(IDL.Nat64),
    'count' : IDL.Nat32,
    'filter' : IDL.Opt(GetLogMessagesFilter),
  });
  const CanisterLogRequest = IDL.Variant({
    'getMessagesInfo' : IDL.Null,
    'getMessages' : GetLogMessagesParameters,
    'getLatestMessages' : GetLatestLogMessagesParameters,
  });
  const GetInformationRequest = IDL.Record({
    'status' : IDL.Opt(StatusRequest),
    'metrics' : IDL.Opt(MetricsRequest),
    'logs' : IDL.Opt(CanisterLogRequest),
    'version' : IDL.Bool,
  });
  const StatusResponse = IDL.Record({
    'memory_size' : IDL.Opt(IDL.Nat64),
    'cycles' : IDL.Opt(IDL.Nat64),
    'heap_memory_size' : IDL.Opt(IDL.Nat64),
  });
  const HourlyMetricsData = IDL.Record({
    'updateCalls' : IDL.Vec(IDL.Nat64),
    'canisterHeapMemorySize' : IDL.Vec(IDL.Nat64),
    'canisterCycles' : IDL.Vec(IDL.Nat64),
    'canisterMemorySize' : IDL.Vec(IDL.Nat64),
    'timeMillis' : IDL.Int,
  });
  const NumericEntity = IDL.Record({
    'avg' : IDL.Nat64,
    'max' : IDL.Nat64,
    'min' : IDL.Nat64,
    'first' : IDL.Nat64,
    'last' : IDL.Nat64,
  });
  const DailyMetricsData = IDL.Record({
    'updateCalls' : IDL.Nat64,
    'canisterHeapMemorySize' : NumericEntity,
    'canisterCycles' : NumericEntity,
    'canisterMemorySize' : NumericEntity,
    'timeMillis' : IDL.Int,
  });
  const CanisterMetricsData = IDL.Variant({
    'hourly' : IDL.Vec(HourlyMetricsData),
    'daily' : IDL.Vec(DailyMetricsData),
  });
  const CanisterMetrics = IDL.Record({ 'data' : CanisterMetricsData });
  const MetricsResponse = IDL.Record({ 'metrics' : IDL.Opt(CanisterMetrics) });
  const CanisterLogFeature = IDL.Variant({
    'filterMessageByContains' : IDL.Null,
    'filterMessageByRegex' : IDL.Null,
  });
  const CanisterLogMessagesInfo = IDL.Record({
    'features' : IDL.Vec(IDL.Opt(CanisterLogFeature)),
    'lastTimeNanos' : IDL.Opt(IDL.Nat64),
    'count' : IDL.Nat32,
    'firstTimeNanos' : IDL.Opt(IDL.Nat64),
  });
  const LogMessageData = IDL.Record({
    'timeNanos' : IDL.Nat64,
    'message' : IDL.Text,
  });
  const CanisterLogMessages = IDL.Record({
    'data' : IDL.Vec(LogMessageData),
    'lastAnalyzedMessageTimeNanos' : IDL.Opt(IDL.Nat64),
  });
  const CanisterLogResponse = IDL.Variant({
    'messagesInfo' : CanisterLogMessagesInfo,
    'messages' : CanisterLogMessages,
  });
  const GetInformationResponse = IDL.Record({
    'status' : IDL.Opt(StatusResponse),
    'metrics' : IDL.Opt(MetricsResponse),
    'logs' : IDL.Opt(CanisterLogResponse),
    'version' : IDL.Opt(IDL.Nat),
  });
  const GetProviderBatchRequest = IDL.Record({ 'ids' : IDL.Vec(IDL.Text) });
  const Status = IDL.Variant({ 'Active' : IDL.Null, 'Suspended' : IDL.Null });
  const V1 = IDL.Record({
    'updated_at' : IDL.Nat64,
    'provider_principal' : IDL.Principal,
    'internal_id' : IDL.Text,
    'display_name' : IDL.Text,
    'session' : IDL.Nat64,
    'address' : IDL.Text,
    'registered_at' : IDL.Nat64,
    'activation_status' : Status,
  });
  const Provider = IDL.Variant({ 'V1' : V1 });
  const GetProviderBatchResponse = IDL.Record({
    'providers' : IDL.Vec(Provider),
  });
  const ProviderInfoRequest = IDL.Record({
    'provider' : IDL.Vec(IDL.Principal),
  });
  const ProviderInfoResponse = IDL.Record({ 'providers' : IDL.Vec(Provider) });
  const GetProviderListRequest = IDL.Record({
    'page' : IDL.Nat64,
    'limit' : IDL.Nat64,
  });
  const GetProviderListResponse = IDL.Record({
    'total_pages' : IDL.Nat64,
    'total_provider_count' : IDL.Nat64,
    'providers' : IDL.Vec(Provider),
  });
  const EmrFragment = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const IssueEmrRequest = IDL.Record({
    'emr' : IDL.Vec(EmrFragment),
    'user_id' : IDL.Text,
  });
  const Header = IDL.Record({
    'provider_id' : IDL.Text,
    'user_id' : IDL.Text,
    'emr_id' : IDL.Text,
    'registry_id' : IDL.Principal,
  });
  const IssueEmrResponse = IDL.Record({ 'emr_header' : Header });
  const PingResult = IDL.Record({
    'patient_registry_status' : IDL.Bool,
    'emr_registry_status' : IDL.Bool,
  });
  const RegisternewProviderRequest = IDL.Record({
    'provider_principal' : IDL.Principal,
    'display_name' : IDL.Text,
    'address' : IDL.Text,
  });
  const SuspendRequest = IDL.Record({ 'principal' : IDL.Principal });
  const CollectMetricsRequestType = IDL.Variant({
    'force' : IDL.Null,
    'normal' : IDL.Null,
  });
  const UpdateInformationRequest = IDL.Record({
    'metrics' : IDL.Opt(CollectMetricsRequestType),
  });
  const EmrHeader = IDL.Record({
    'provider_id' : IDL.Text,
    'user_id' : IDL.Text,
    'emr_id' : IDL.Text,
    'registry_id' : IDL.Principal,
  });
  const UpdateEmrRequest = IDL.Record({
    'fields' : IDL.Vec(EmrFragment),
    'header' : EmrHeader,
  });
  return IDL.Service({
    'add_authorized_metrics_collector' : IDL.Func(
        [AuthorizedCallerRequest],
        [],
        [],
      ),
    'emr_list_provider' : IDL.Func(
        [EmrListProviderRequest],
        [EmrListProviderResponse],
        ['query'],
      ),
    'getCanistergeekInformation' : IDL.Func(
        [GetInformationRequest],
        [GetInformationResponse],
        ['query'],
      ),
    'get_provider_batch' : IDL.Func(
        [GetProviderBatchRequest],
        [GetProviderBatchResponse],
        ['query'],
      ),
    'get_provider_info_with_principal' : IDL.Func(
        [ProviderInfoRequest],
        [ProviderInfoResponse],
        ['query'],
      ),
    'get_provider_list' : IDL.Func(
        [GetProviderListRequest],
        [GetProviderListResponse],
        ['query'],
      ),
    'get_trusted_origins' : IDL.Func([], [IDL.Vec(IDL.Text)], []),
    'is_valid_provider' : IDL.Func([IDL.Principal], [IDL.Bool], ['query']),
    'issue_emr' : IDL.Func([IssueEmrRequest], [IssueEmrResponse], []),
    'metrics' : IDL.Func([], [IDL.Text], ['query']),
    'ping' : IDL.Func([], [PingResult], ['composite_query']),
    'register_new_provider' : IDL.Func(
        [RegisternewProviderRequest],
        [IDL.Record({})],
        [],
      ),
    'remove_authorized_metrics_collector' : IDL.Func(
        [AuthorizedCallerRequest],
        [],
        [],
      ),
    'suspend_provider' : IDL.Func([SuspendRequest], [], []),
    'unsuspend_provider' : IDL.Func([SuspendRequest], [], []),
    'updateCanistergeekInformation' : IDL.Func(
        [UpdateInformationRequest],
        [],
        [],
      ),
    'update_emr' : IDL.Func([UpdateEmrRequest], [IDL.Record({})], []),
    'update_emr_registry_principal' : IDL.Func([SuspendRequest], [], []),
    'update_patient_registry_principal' : IDL.Func([SuspendRequest], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
