export const idlFactory = ({ IDL }) => {
  const AuthorizedCallerRequest = IDL.Record({ 'caller' : IDL.Principal });
  const ClaimConsentRequest = IDL.Record({ 'code' : IDL.Text });
  const ClaimConsentResponse = IDL.Record({ 'session_id' : IDL.Text });
  const EmrListPatientRequest = IDL.Record({
    'page' : IDL.Nat8,
    'limit' : IDL.Nat8,
  });
  const EmrHeader = IDL.Record({
    'provider_id' : IDL.Text,
    'user_id' : IDL.Text,
    'emr_id' : IDL.Text,
    'registry_id' : IDL.Principal,
  });
  const EmrListPatientResponse = IDL.Record({ 'emrs' : IDL.Vec(EmrHeader) });
  const EmrListConsentRequest = IDL.Record({
    'session_id' : IDL.Text,
    'page' : IDL.Nat8,
    'limit' : IDL.Nat8,
  });
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
  const V1 = IDL.Record({
    'martial_status' : IDL.Text,
    'place_of_birth' : IDL.Text,
    'address' : IDL.Text,
    'gender' : IDL.Text,
    'date_of_birth' : IDL.Text,
  });
  const Patient = IDL.Variant({ 'V1' : V1 });
  const GetPatientInfoResponse = IDL.Record({ 'patient' : Patient });
  const IsConsentClaimedResponse = IDL.Record({ 'claimed' : IDL.Bool });
  const IssueRequest = IDL.Record({ 'header' : EmrHeader });
  const PatientListResponse = IDL.Record({ 'patients' : IDL.Vec(Patient) });
  const PingResult = IDL.Record({ 'emr_registry_status' : IDL.Bool });
  const ReadEmrByIdRequest = IDL.Record({
    'provider_id' : IDL.Text,
    'emr_id' : IDL.Text,
    'registry_id' : IDL.Principal,
  });
  const EmrFragment = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const EmrHeaderWithBody = IDL.Record({
    'body' : IDL.Vec(EmrFragment),
    'header' : EmrHeader,
  });
  const ReadEmrByIdResponse = IDL.Record({ 'emr' : EmrHeaderWithBody });
  const ReadEmrSessionRequest = IDL.Record({
    'session_id' : IDL.Text,
    'args' : ReadEmrByIdRequest,
  });
  const RegisterPatientRequest = IDL.Record({ 'nik' : IDL.Text });
  const CollectMetricsRequestType = IDL.Variant({
    'force' : IDL.Null,
    'normal' : IDL.Null,
  });
  const UpdateInformationRequest = IDL.Record({
    'metrics' : IDL.Opt(CollectMetricsRequestType),
  });
  const UpdateEmrRegistryRequest = IDL.Record({ 'principal' : IDL.Principal });
  const UpdateInitialPatientInfoRequest = IDL.Record({ 'info' : V1 });
  return IDL.Service({
    'add_authorized_metrics_collector' : IDL.Func(
        [AuthorizedCallerRequest],
        [],
        [],
      ),
    'claim_consent' : IDL.Func(
        [ClaimConsentRequest],
        [ClaimConsentResponse],
        [],
      ),
    'create_consent' : IDL.Func([], [ClaimConsentRequest], []),
    'emr_list_patient' : IDL.Func(
        [EmrListPatientRequest],
        [EmrListPatientResponse],
        ['query'],
      ),
    'emr_list_with_session' : IDL.Func(
        [EmrListConsentRequest],
        [EmrListPatientResponse],
        ['query'],
      ),
    'finish_session' : IDL.Func([ClaimConsentResponse], [], []),
    'getCanistergeekInformation' : IDL.Func(
        [GetInformationRequest],
        [GetInformationResponse],
        ['query'],
      ),
    'get_patient_info' : IDL.Func([], [GetPatientInfoResponse], ['query']),
    'get_patient_info_with_consent' : IDL.Func(
        [ClaimConsentResponse],
        [GetPatientInfoResponse],
        ['query'],
      ),
    'get_trusted_origins' : IDL.Func([], [IDL.Vec(IDL.Text)], []),
    'is_consent_claimed' : IDL.Func(
        [ClaimConsentRequest],
        [IsConsentClaimedResponse],
        ['query'],
      ),
    'metrics' : IDL.Func([], [IDL.Text], ['query']),
    'notify_issued' : IDL.Func([IssueRequest], [], []),
    'patient_list' : IDL.Func([], [PatientListResponse], ['query']),
    'ping' : IDL.Func([], [PingResult], ['composite_query']),
    'read_emr_by_id' : IDL.Func(
        [ReadEmrByIdRequest],
        [ReadEmrByIdResponse],
        ['composite_query'],
      ),
    'read_emr_with_session' : IDL.Func(
        [ReadEmrSessionRequest],
        [ReadEmrByIdResponse],
        ['composite_query'],
      ),
    'register_patient' : IDL.Func([RegisterPatientRequest], [], []),
    'remove_authorized_metrics_collector' : IDL.Func(
        [AuthorizedCallerRequest],
        [],
        [],
      ),
    'revoke_consent' : IDL.Func([ClaimConsentRequest], [], []),
    'updateCanistergeekInformation' : IDL.Func(
        [UpdateInformationRequest],
        [],
        [],
      ),
    'update_emr_registry_principal' : IDL.Func(
        [UpdateEmrRegistryRequest],
        [],
        [],
      ),
    'update_initial_patient_info' : IDL.Func(
        [UpdateInitialPatientInfoRequest],
        [],
        [],
      ),
    'update_provider_registry_principal' : IDL.Func(
        [UpdateEmrRegistryRequest],
        [],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
