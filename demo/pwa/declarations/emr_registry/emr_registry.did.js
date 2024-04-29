export const idlFactory = ({ IDL }) => {
  const AuthorizedCallerRequest = IDL.Record({ caller: IDL.Principal });
  const EmrFragment = IDL.Record({ key: IDL.Text, value: IDL.Text });
  const CreateEmrRequest = IDL.Record({
    emr: IDL.Vec(EmrFragment),
    provider_id: IDL.Text,
    user_id: IDL.Text,
    emr_id: IDL.Text,
  });
  const Header = IDL.Record({
    provider_id: IDL.Text,
    user_id: IDL.Text,
    emr_id: IDL.Text,
    registry_id: IDL.Principal,
  });
  const CreateEmrResponse = IDL.Record({ header: Header });
  const StatusRequest = IDL.Record({
    memory_size: IDL.Bool,
    cycles: IDL.Bool,
    heap_memory_size: IDL.Bool,
  });
  const MetricsGranularity = IDL.Variant({
    hourly: IDL.Null,
    daily: IDL.Null,
  });
  const GetMetricsParameters = IDL.Record({
    dateToMillis: IDL.Nat,
    granularity: MetricsGranularity,
    dateFromMillis: IDL.Nat,
  });
  const MetricsRequest = IDL.Record({ parameters: GetMetricsParameters });
  const GetLogMessagesFilter = IDL.Record({
    analyzeCount: IDL.Nat32,
    messageRegex: IDL.Opt(IDL.Text),
    messageContains: IDL.Opt(IDL.Text),
  });
  const GetLogMessagesParameters = IDL.Record({
    count: IDL.Nat32,
    filter: IDL.Opt(GetLogMessagesFilter),
    fromTimeNanos: IDL.Opt(IDL.Nat64),
  });
  const GetLatestLogMessagesParameters = IDL.Record({
    upToTimeNanos: IDL.Opt(IDL.Nat64),
    count: IDL.Nat32,
    filter: IDL.Opt(GetLogMessagesFilter),
  });
  const CanisterLogRequest = IDL.Variant({
    getMessagesInfo: IDL.Null,
    getMessages: GetLogMessagesParameters,
    getLatestMessages: GetLatestLogMessagesParameters,
  });
  const GetInformationRequest = IDL.Record({
    status: IDL.Opt(StatusRequest),
    metrics: IDL.Opt(MetricsRequest),
    logs: IDL.Opt(CanisterLogRequest),
    version: IDL.Bool,
  });
  const StatusResponse = IDL.Record({
    memory_size: IDL.Opt(IDL.Nat64),
    cycles: IDL.Opt(IDL.Nat64),
    heap_memory_size: IDL.Opt(IDL.Nat64),
  });
  const HourlyMetricsData = IDL.Record({
    updateCalls: IDL.Vec(IDL.Nat64),
    canisterHeapMemorySize: IDL.Vec(IDL.Nat64),
    canisterCycles: IDL.Vec(IDL.Nat64),
    canisterMemorySize: IDL.Vec(IDL.Nat64),
    timeMillis: IDL.Int,
  });
  const NumericEntity = IDL.Record({
    avg: IDL.Nat64,
    max: IDL.Nat64,
    min: IDL.Nat64,
    first: IDL.Nat64,
    last: IDL.Nat64,
  });
  const DailyMetricsData = IDL.Record({
    updateCalls: IDL.Nat64,
    canisterHeapMemorySize: NumericEntity,
    canisterCycles: NumericEntity,
    canisterMemorySize: NumericEntity,
    timeMillis: IDL.Int,
  });
  const CanisterMetricsData = IDL.Variant({
    hourly: IDL.Vec(HourlyMetricsData),
    daily: IDL.Vec(DailyMetricsData),
  });
  const CanisterMetrics = IDL.Record({ data: CanisterMetricsData });
  const MetricsResponse = IDL.Record({ metrics: IDL.Opt(CanisterMetrics) });
  const CanisterLogFeature = IDL.Variant({
    filterMessageByContains: IDL.Null,
    filterMessageByRegex: IDL.Null,
  });
  const CanisterLogMessagesInfo = IDL.Record({
    features: IDL.Vec(IDL.Opt(CanisterLogFeature)),
    lastTimeNanos: IDL.Opt(IDL.Nat64),
    count: IDL.Nat32,
    firstTimeNanos: IDL.Opt(IDL.Nat64),
  });
  const LogMessageData = IDL.Record({
    timeNanos: IDL.Nat64,
    message: IDL.Text,
  });
  const CanisterLogMessages = IDL.Record({
    data: IDL.Vec(LogMessageData),
    lastAnalyzedMessageTimeNanos: IDL.Opt(IDL.Nat64),
  });
  const CanisterLogResponse = IDL.Variant({
    messagesInfo: CanisterLogMessagesInfo,
    messages: CanisterLogMessages,
  });
  const GetInformationResponse = IDL.Record({
    status: IDL.Opt(StatusResponse),
    metrics: IDL.Opt(MetricsResponse),
    logs: IDL.Opt(CanisterLogResponse),
    version: IDL.Opt(IDL.Nat),
  });
  const ReadEmrByIdRequest = IDL.Record({
    provider_id: IDL.Text,
    user_id: IDL.Text,
    emr_id: IDL.Text,
  });
  const EmrHeaderWithBody = IDL.Record({
    body: IDL.Vec(EmrFragment),
    header: Header,
  });
  const ReadEmrByIdResponse = IDL.Record({ emr: EmrHeaderWithBody });
  const RemoveEmrRequest = IDL.Record({ header: Header });
  const RemoveEmrResponse = IDL.Record({ status: IDL.Bool });
  const CollectMetricsRequestType = IDL.Variant({
    force: IDL.Null,
    normal: IDL.Null,
  });
  const UpdateInformationRequest = IDL.Record({
    metrics: IDL.Opt(CollectMetricsRequestType),
  });
  const UpdateEmrRequest = IDL.Record({
    fields: IDL.Vec(EmrFragment),
    header: Header,
  });
  return IDL.Service({
    add_authorized_caller: IDL.Func([AuthorizedCallerRequest], [], []),
    add_authorized_metrics_collector: IDL.Func(
      [AuthorizedCallerRequest],
      [],
      [],
    ),
    create_emr: IDL.Func([CreateEmrRequest], [CreateEmrResponse], []),
    getCanistergeekInformation: IDL.Func(
      [GetInformationRequest],
      [GetInformationResponse],
      ['query'],
    ),
    metrics: IDL.Func([], [IDL.Text], ['query']),
    ping: IDL.Func([], [], ['query']),
    read_emr_by_id: IDL.Func(
      [ReadEmrByIdRequest],
      [ReadEmrByIdResponse],
      ['query'],
    ),
    remove_authorized_caller: IDL.Func([AuthorizedCallerRequest], [], []),
    remove_authorized_metrics_collector: IDL.Func(
      [AuthorizedCallerRequest],
      [],
      [],
    ),
    remove_emr: IDL.Func([RemoveEmrRequest], [RemoveEmrResponse], []),
    updateCanistergeekInformation: IDL.Func([UpdateInformationRequest], [], []),
    update_emr: IDL.Func([UpdateEmrRequest], [RemoveEmrRequest], []),
  });
};
export const init = ({ IDL }) => {
  return [];
};
