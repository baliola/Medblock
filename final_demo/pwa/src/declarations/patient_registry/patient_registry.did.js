export const idlFactory = ({ IDL }) => {
  const AuthorizedCallerRequest = IDL.Record({ 'caller': IDL.Principal });
  const ClaimConsentRequest = IDL.Record({ 'code': IDL.Text });
  const ClaimConsentResponse = IDL.Record({
    'session_id': IDL.Text,
    'name': IDL.Text,
  });
  const Consent = IDL.Record({
    'nik': IDL.Text,
    'session_id': IDL.Opt(IDL.Text),
    'code': IDL.Text,
    'claimed': IDL.Bool,
    'session_user': IDL.Opt(IDL.Text),
  });
  const ConsentListResponse = IDL.Record({ 'consents': IDL.Vec(Consent) });
  const EmrListPatientRequest = IDL.Record({
    'page': IDL.Nat8,
    'limit': IDL.Nat8,
  });
  const HeaderStatus = IDL.Record({
    'updated_at': IDL.Nat64,
    'created_at': IDL.Nat64,
  });
  const EmrHeader = IDL.Record({
    'provider_id': IDL.Text,
    'user_id': IDL.Text,
    'emr_id': IDL.Text,
    'registry_id': IDL.Principal,
  });
  const EmrHeaderWithStatus = IDL.Record({
    'status': HeaderStatus,
    'hospital_name': IDL.Text,
    'header': EmrHeader,
  });
  const EmrListPatientResponse = IDL.Record({
    'emrs': IDL.Vec(EmrHeaderWithStatus),
  });
  const EmrListConsentRequest = IDL.Record({
    'session_id': IDL.Text,
    'page': IDL.Nat8,
    'limit': IDL.Nat8,
  });
  const EmrListConsentResponse = IDL.Record({
    'emr': IDL.Vec(EmrHeaderWithStatus),
    'username': IDL.Text,
  });
  const FinishSessionRequest = IDL.Record({ 'session_id': IDL.Text });
  const StatusRequest = IDL.Record({
    'memory_size': IDL.Bool,
    'cycles': IDL.Bool,
    'heap_memory_size': IDL.Bool,
  });
  const MetricsGranularity = IDL.Variant({
    'hourly': IDL.Null,
    'daily': IDL.Null,
  });
  const GetMetricsParameters = IDL.Record({
    'dateToMillis': IDL.Nat,
    'granularity': MetricsGranularity,
    'dateFromMillis': IDL.Nat,
  });
  const MetricsRequest = IDL.Record({ 'parameters': GetMetricsParameters });
  const GetLogMessagesFilter = IDL.Record({
    'analyzeCount': IDL.Nat32,
    'messageRegex': IDL.Opt(IDL.Text),
    'messageContains': IDL.Opt(IDL.Text),
  });
  const GetLogMessagesParameters = IDL.Record({
    'count': IDL.Nat32,
    'filter': IDL.Opt(GetLogMessagesFilter),
    'fromTimeNanos': IDL.Opt(IDL.Nat64),
  });
  const GetLatestLogMessagesParameters = IDL.Record({
    'upToTimeNanos': IDL.Opt(IDL.Nat64),
    'count': IDL.Nat32,
    'filter': IDL.Opt(GetLogMessagesFilter),
  });
  const CanisterLogRequest = IDL.Variant({
    'getMessagesInfo': IDL.Null,
    'getMessages': GetLogMessagesParameters,
    'getLatestMessages': GetLatestLogMessagesParameters,
  });
  const GetInformationRequest = IDL.Record({
    'status': IDL.Opt(StatusRequest),
    'metrics': IDL.Opt(MetricsRequest),
    'logs': IDL.Opt(CanisterLogRequest),
    'version': IDL.Bool,
  });
  const StatusResponse = IDL.Record({
    'memory_size': IDL.Opt(IDL.Nat64),
    'cycles': IDL.Opt(IDL.Nat64),
    'heap_memory_size': IDL.Opt(IDL.Nat64),
  });
  const HourlyMetricsData = IDL.Record({
    'updateCalls': IDL.Vec(IDL.Nat64),
    'canisterHeapMemorySize': IDL.Vec(IDL.Nat64),
    'canisterCycles': IDL.Vec(IDL.Nat64),
    'canisterMemorySize': IDL.Vec(IDL.Nat64),
    'timeMillis': IDL.Int,
  });
  const NumericEntity = IDL.Record({
    'avg': IDL.Nat64,
    'max': IDL.Nat64,
    'min': IDL.Nat64,
    'first': IDL.Nat64,
    'last': IDL.Nat64,
  });
  const DailyMetricsData = IDL.Record({
    'updateCalls': IDL.Nat64,
    'canisterHeapMemorySize': NumericEntity,
    'canisterCycles': NumericEntity,
    'canisterMemorySize': NumericEntity,
    'timeMillis': IDL.Int,
  });
  const CanisterMetricsData = IDL.Variant({
    'hourly': IDL.Vec(HourlyMetricsData),
    'daily': IDL.Vec(DailyMetricsData),
  });
  const CanisterMetrics = IDL.Record({ 'data': CanisterMetricsData });
  const MetricsResponse = IDL.Record({ 'metrics': IDL.Opt(CanisterMetrics) });
  const CanisterLogFeature = IDL.Variant({
    'filterMessageByContains': IDL.Null,
    'filterMessageByRegex': IDL.Null,
  });
  const CanisterLogMessagesInfo = IDL.Record({
    'features': IDL.Vec(IDL.Opt(CanisterLogFeature)),
    'lastTimeNanos': IDL.Opt(IDL.Nat64),
    'count': IDL.Nat32,
    'firstTimeNanos': IDL.Opt(IDL.Nat64),
  });
  const LogMessageData = IDL.Record({
    'timeNanos': IDL.Nat64,
    'message': IDL.Text,
  });
  const CanisterLogMessages = IDL.Record({
    'data': IDL.Vec(LogMessageData),
    'lastAnalyzedMessageTimeNanos': IDL.Opt(IDL.Nat64),
  });
  const CanisterLogResponse = IDL.Variant({
    'messagesInfo': CanisterLogMessagesInfo,
    'messages': CanisterLogMessages,
  });
  const GetInformationResponse = IDL.Record({
    'status': IDL.Opt(StatusResponse),
    'metrics': IDL.Opt(MetricsResponse),
    'logs': IDL.Opt(CanisterLogResponse),
    'version': IDL.Opt(IDL.Nat),
  });
  const ActivityType = IDL.Variant({
    'Updated': IDL.Null,
    'Accessed': IDL.Null,
    'Revoked': IDL.Null,
  });
  const Activity = IDL.Record({
    'activity_type': ActivityType,
    'provider_id': IDL.Text,
    'user_id': IDL.Text,
    'timestamp': IDL.Nat64,
  });
  const LogResponse = IDL.Record({ 'logs': IDL.Vec(Activity) });
  const V1 = IDL.Record({
    'name': IDL.Text,
    'martial_status': IDL.Text,
    'place_of_birth': IDL.Text,
    'address': IDL.Text,
    'gender': IDL.Text,
    'date_of_birth': IDL.Text,
  });
  const Patient = IDL.Variant({ 'V1': V1 });
  const GetPatientInfoResponse = IDL.Record({
    'nik': IDL.Text,
    'patient': Patient,
  });
  const IsConsentClaimedResponse = IDL.Record({
    'info': IDL.Opt(Consent),
    'claimed': IDL.Bool,
  });
  const IssueRequest = IDL.Record({ 'header': EmrHeader });
  const PatientWithNikAndSession = IDL.Record({
    'nik': IDL.Text,
    'session_id': IDL.Text,
    'info': Patient,
  });
  const PatientListResponse = IDL.Record({
    'patients': IDL.Vec(PatientWithNikAndSession),
  });
  const PingResult = IDL.Record({ 'emr_registry_status': IDL.Bool });
  const ReadEmrByIdRequest = IDL.Record({
    'provider_id': IDL.Text,
    'emr_id': IDL.Text,
    'registry_id': IDL.Principal,
  });
  const EmrFragment = IDL.Record({ 'key': IDL.Text, 'value': IDL.Text });
  const EmrHeaderWithBody = IDL.Record({
    'body': IDL.Vec(EmrFragment),
    'header': EmrHeader,
  });
  const ReadEmrByIdResponse = IDL.Record({ 'emr': EmrHeaderWithBody });
  const ReadEmrSessionRequest = IDL.Record({
    'session_id': IDL.Text,
    'args': ReadEmrByIdRequest,
  });
  const RegisterPatientRequest = IDL.Record({ 'nik': IDL.Text });
  const RevokeConsentRequest = IDL.Record({ 'codes': IDL.Vec(IDL.Text) });
  const SearchPatientRequest = IDL.Record({ 'nik': IDL.Text });
  const SearchPatientResponse = IDL.Record({
    'patient_info': PatientWithNikAndSession,
  });
  const CollectMetricsRequestType = IDL.Variant({
    'force': IDL.Null,
    'normal': IDL.Null,
  });
  const UpdateInformationRequest = IDL.Record({
    'metrics': IDL.Opt(CollectMetricsRequestType),
  });
  const UpdateEmrRegistryRequest = IDL.Record({ 'principal': IDL.Principal });
  const UpdateInitialPatientInfoRequest = IDL.Record({ 'info': V1 });
  return IDL.Service({
    'add_authorized_metrics_collector': IDL.Func(
      [AuthorizedCallerRequest],
      [],
      [],
    ),
    'claim_consent': IDL.Func(
      [ClaimConsentRequest],
      [ClaimConsentResponse],
      [],
    ),
    'consent_list': IDL.Func([], [ConsentListResponse], ['query']),
    'create_consent': IDL.Func([], [ClaimConsentRequest], []),
    'emr_list_patient': IDL.Func(
      [EmrListPatientRequest],
      [EmrListPatientResponse],
      ['composite_query'],
    ),
    'emr_list_with_session': IDL.Func(
      [EmrListConsentRequest],
      [EmrListConsentResponse],
      ['composite_query'],
    ),
    'finish_session': IDL.Func([FinishSessionRequest], [], []),
    'getCanistergeekInformation': IDL.Func(
      [GetInformationRequest],
      [GetInformationResponse],
      ['query'],
    ),
    'get_logs': IDL.Func([], [LogResponse], ['query']),
    'get_patient_info': IDL.Func([], [GetPatientInfoResponse], ['query']),
    'get_patient_info_with_consent': IDL.Func(
      [FinishSessionRequest],
      [GetPatientInfoResponse],
      ['composite_query'],
    ),
    'get_trusted_origins': IDL.Func([], [IDL.Vec(IDL.Text)], []),
    'is_consent_claimed': IDL.Func(
      [ClaimConsentRequest],
      [IsConsentClaimedResponse],
      ['query'],
    ),
    'metrics': IDL.Func([], [IDL.Text], ['query']),
    'notify_issued': IDL.Func([IssueRequest], [], []),
    'notify_updated': IDL.Func([IssueRequest], [], []),
    'patient_list': IDL.Func([], [PatientListResponse], ['composite_query']),
    'ping': IDL.Func([], [PingResult], ['composite_query']),
    'read_emr_by_id': IDL.Func(
      [ReadEmrByIdRequest],
      [ReadEmrByIdResponse],
      ['composite_query'],
    ),
    'read_emr_with_session': IDL.Func(
      [ReadEmrSessionRequest],
      [ReadEmrByIdResponse],
      ['composite_query'],
    ),
    'register_patient': IDL.Func([RegisterPatientRequest], [], []),
    'remove_authorized_metrics_collector': IDL.Func(
      [AuthorizedCallerRequest],
      [],
      [],
    ),
    'revoke_consent': IDL.Func([RevokeConsentRequest], [], []),
    'search_patient': IDL.Func(
      [SearchPatientRequest],
      [SearchPatientResponse],
      ['composite_query'],
    ),
    'updateCanistergeekInformation': IDL.Func(
      [UpdateInformationRequest],
      [],
      [],
    ),
    'update_emr_registry_principal': IDL.Func(
      [UpdateEmrRegistryRequest],
      [],
      [],
    ),
    'update_initial_patient_info': IDL.Func(
      [UpdateInitialPatientInfoRequest],
      [],
      [],
    ),
    'update_provider_registry_principal': IDL.Func(
      [UpdateEmrRegistryRequest],
      [],
      [],
    ),
  });
};
export const init = ({ IDL }) => { return []; };
