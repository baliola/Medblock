use std::{borrow::Borrow, cell::RefCell, time::Duration};

use api::{
    AuthorizedCallerRequest, GetProviderBatchRequest, GetProviderBatchResponse,
    GetProviderListRequest, GetProviderListResponse, IssueEmrResponse, PingResult,
    ProviderInfoRequest, ProviderInfoResponse, RegisternewProviderRequest,
    RegisternewProviderResponse, SuspendRequest, UnSuspendRequest, UpdateEmrRegistryRequest,
    UpdatePatientRegistryRequest,
};
use candid::{Decode, Encode, Principal};
use canister_common::{
    common::{freeze::FreezeThreshold, guard::verified_caller},
    id_generator::IdGenerator,
    log,
    mmgr::MemoryManager,
    random::CanisterRandomSource,
    register_log,
    stable::{Candid, Memory, Stable},
    statistics::{self, traits::OpaqueMetrics},
};

use ic_stable_structures::Cell;
use memory::{FreezeThresholdMemory, UpgradeMemory};
use registry::ProviderRegistry;

pub mod api;
mod config;
mod declarations;
mod memory;
mod registry;
mod types;

/// TODO: benchmark this
const CANISTER_CYCLE_THRESHOLD: u128 = 300_000;

// change this if you want to change the interval of the metrics collection
const METRICS_INTERVAL: Duration = Duration::from_secs(60 * 5); // 5 minutes

pub struct State {
    providers: ProviderRegistry,
    config: Cell<Stable<config::CanisterConfig, Candid>, Memory>,
    memory_manager: MemoryManager,
    freeze_threshold: Cell<Stable<FreezeThreshold, Candid>, Memory>,
}

register_log!("provider");
thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
    static ID_GENERATOR: RefCell<Option<IdGenerator<CanisterRandomSource>>> = const {
        RefCell::new(None)
    };
}

/// A helper method to read the state.
///
/// Precondition: the state is already initialized.
pub fn with_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(cell.borrow().as_ref().expect("state not initialized")))
}

/// A helper method to read the id generator.
///
/// Precondition: the id generator is already initialized.
pub fn with_id_generator_mut<R>(f: impl FnOnce(&mut IdGenerator<CanisterRandomSource>) -> R) -> R {
    ID_GENERATOR.with(|cell| {
        f(cell
            .borrow_mut()
            .as_mut()
            .expect("id generator not initialized"))
    })
}

/// A helper method to mutate the state.
///
/// Precondition: the state is already initialized.
pub fn with_state_mut<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(cell.borrow_mut().as_mut().expect("state not initialized")))
}

#[ic_cdk::update]
async fn get_trusted_origins() -> Vec<String> {
    vec![
        // Origins should be in the format defined by the Window.postMessage method (https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage#the_dispatched_event)
        String::from("http://bw4dl-smaaa-aaaaa-qaacq-cai.localhost:4943"), // to be replaced with your frontend origin(s) // TODO: make a config out of this
        String::from("http://localhost:3000"),
        String::from("https://demo-app.medblock.id"),
        String::from("https://demo-web.medblock.id"),
        String::from("http://54.255.210.149:3001"),
        String::from("http://54.255.210.149:3000"),
        String::from("https://bwvkymxvy2stchdh.medblock.id"),
        String::from("https://bwvkymxvy2std2vi.medblock.id"),
        String::from("https://web-app.medblock.id"),
        String::from("https://app.medblock.id"),
        String::from("https://webadmin.medblock.id/"),
        String::from("https://d2viywrtiw4k.medblock.id"),
        String::from("https://d2viywrtiw4k.medblock.id/"),
        String::from("https://staging.d1f976xml8qycj.amplifyapp.com/"),
        String::from("https://staging.d1cbmngfhpp9l5.amplifyapp.com/"),
    ]
}

// guard function
fn only_canister_owner() -> Result<(), String> {
    let caller = verified_caller()?;

    match ic_cdk::api::is_controller(&caller) {
        true => Ok(()),
        false => Err(
            "[PROVIDER_REGISTRY_LIB] Only canister controller can call this method. You need to register as Provider Registry Canister Owner to call this method.".to_string(),
        ),
    }
}

// guard function
fn only_provider() -> Result<(), String> {
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let caller = verified_caller()?;

        if !state.providers.is_valid_provider(&caller) {
            return Err(
                "[PROVIDER_REGISTRY_LIB] Only provider can call this method. Is your principal registered as provider?".to_string(),
            );
        }

        // safe to unwrap as we already check if caller is a valid provider or not
        if state.providers.is_provider_suspended(&caller).unwrap() {
            return Err("[PROVIDER_REGISTRY_LIB] Provider is suspended.".to_string());
        }

        Ok(())
    })
}

// guard function
fn only_authorized_metrics_collector() -> Result<(), String> {
    let caller = verified_caller()?;

    with_state(|s| {
        if !s.config.get().is_authorized_metrics_collector(&caller) {
            return Err(
                "[PROVIDER_REGISTRY_LIB] Only authorized metrics collector can call this method."
                    .to_string(),
            );
        }

        Ok(())
    })
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    serialize_canister_metrics();
}

fn serialize_canister_metrics() {
    let monitor_stable_data = canistergeek_ic_rust::monitor::pre_upgrade_stable_data();
    let logger_stable_data = canistergeek_ic_rust::logger::pre_upgrade_stable_data();

    let mut mem = with_state(|s| s.memory_manager.get_memory::<_, UpgradeMemory>(|mem| mem));
    let Ok(encoded) = Encode!(&monitor_stable_data, &logger_stable_data) else {
        return;
    };

    let encoded_len = encoded.len();
    let mut writer = ic_stable_structures::writer::Writer::new(&mut mem, 0);

    let write = writer.write(&encoded_len.to_le_bytes());
    match write {
        Ok(_) => {
            log!(
                "encoded canister metrics length written successfully : {} bytes",
                encoded_len
            );
        }

        Err(e) => {
            log!(
                "OOM ERROR: failed to write encoded canister metrics length {:?}",
                e
            );
        }
    }

    let write = writer.write(&encoded);
    match write {
        Ok(_) => {
            log!("encoded canister metrics written");
        }
        Err(e) => {
            log!(
                "OOM ERROR: failed to write encoded canister metrics {:?}",
                e
            );
        }
    }
}

fn start_collect_metrics_job() {
    ic_cdk_timers::set_timer_interval(METRICS_INTERVAL, || {
        log!("updating metrics");

        canistergeek_ic_rust::update_information(
            canistergeek_ic_rust::api_type::UpdateInformationRequest {
                metrics: Some(canistergeek_ic_rust::api_type::CollectMetricsRequestType::force),
            },
        );

        log!("metrics updated");
    });
}

fn deserialize_canister_metrics() {
    let mem = with_state(|s| s.memory_manager.get_memory::<_, UpgradeMemory>(|mem| mem));

    let mut reader = ic_stable_structures::reader::Reader::new(&mem, 0);

    let mut len_buf = [0; 4];
    let read_len = reader.read(&mut len_buf).unwrap();
    log!("encoded canister metrics length: {:?} bytes", read_len);

    let mut state_buf = vec![0; u32::from_le_bytes(len_buf) as usize];
    let read_len = reader.read(&mut state_buf).unwrap();
    log!(
        "readed encoded canister metrics length: {:?} bytes",
        read_len
    );

    let (monitor_stable_data, logger_stable_data) = Decode!(
        &state_buf,
        (
            u8,
            std::collections::BTreeMap<u32, canistergeek_ic_rust::monitor::data_type::DayData>
        ),
        (u8, canistergeek_ic_rust::logger::LogMessageStorage)
    )
    .unwrap();

    canistergeek_ic_rust::monitor::post_upgrade_stable_data(monitor_stable_data);
    canistergeek_ic_rust::logger::post_upgrade_stable_data(logger_stable_data);
}

fn initialize_id_generator() {
    ic_cdk_timers::set_timer(Duration::from_secs(3), || {
        ic_cdk::spawn(async move {
            let rng = CanisterRandomSource::new().await;
            let id_generator = IdGenerator::new(rng);

            ID_GENERATOR.with(|id_gen| {
                *id_gen.borrow_mut() = Some(id_generator);
            });

            log!("id generator initialized");
        })
    });
}

fn init_state() -> State {
    let memory_manager = MemoryManager::init();

    State {
        providers: ProviderRegistry::init(&memory_manager),
        config: config::CanisterConfig::init(&memory_manager),
        freeze_threshold: FreezeThreshold::init::<FreezeThresholdMemory>(
            CANISTER_CYCLE_THRESHOLD,
            &memory_manager,
        ),
        memory_manager,
    }
}

fn initialize() {
    let state = init_state();
    STATE.replace(Some(state));
    log!("canister state initialized");
    initialize_id_generator();
    start_collect_metrics_job()
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    initialize();
    deserialize_canister_metrics();
}

#[ic_cdk::init]
fn canister_init() {
    initialize()
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn remove_authorized_metrics_collector(req: AuthorizedCallerRequest) {
    with_state_mut(|s| {
        let mut config = s.config.get().to_owned();

        config.remove_authorized_metrics_collector(req.caller);

        s.config.set(config);
    });
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn add_authorized_metrics_collector(req: AuthorizedCallerRequest) {
    with_state_mut(|s| {
        let mut config = s.config.get().to_owned();

        config.add_authorized_metrics_collector(req.caller);

        s.config.set(config);
    });
}

#[ic_cdk::query(
    guard = "only_authorized_metrics_collector",
    name = "getCanistergeekInformation"
)]
pub async fn canister_geek_metrics(
    request: canistergeek_ic_rust::api_type::GetInformationRequest,
) -> canistergeek_ic_rust::api_type::GetInformationResponse<'static> {
    canistergeek_ic_rust::get_information(request)
}

#[ic_cdk::update(
    guard = "only_authorized_metrics_collector",
    name = "updateCanistergeekInformation"
)]
pub async fn update_canistergeek_information(
    request: canistergeek_ic_rust::api_type::UpdateInformationRequest,
) {
    canistergeek_ic_rust::update_information(request);
}

#[ic_cdk::query(guard = "only_authorized_metrics_collector")]
fn metrics() -> String {
    with_state(|s| {
        let config: config::CanisterConfig = s.config.get().to_owned().into_inner();
        [
            s.providers.measure(),
            statistics::canister::BlockchainMetrics::measure(),
            statistics::canister::MemoryStatistics::measure(),
            <config::CanisterConfig as OpaqueMetrics>::measure(&config),
        ]
        .join("\n")
    })
}

#[ic_cdk::query(guard = "only_provider")]
fn emr_list_provider(req: types::EmrListProviderRequest) -> types::EmrListProviderResponse {
    with_state(|state| {
        let provider = verified_caller().unwrap();

        let limit = state.config.get().max_item_per_response().min(req.limit);

        state
            .providers
            .get_issued(&provider, req.page, limit as u64)
            .unwrap()
            .into()
    })
}

#[ic_cdk::update(guard = "only_provider")]
async fn issue_emr(req: api::IssueEmrRequest) -> api::IssueEmrResponse {
    let emr_id = with_id_generator_mut(|generator| generator.generate_id());
    let args = with_state(|s| s.providers.build_args_call_emr_canister(req, emr_id)).unwrap();

    // safe to unwrap as the provider id comes from canister
    let provider_principal = verified_caller().unwrap();

    let emr_registry = with_state(|s| s.config.get().emr_registry());
    let patient_registry = with_state(|s| s.config.get().patient_registry());

    let response = ProviderRegistry::do_call_create_emr(args, emr_registry, patient_registry).await;

    with_state_mut(|s| {
        s.providers.issue_emr(
            response.header.emr_id.clone().try_into().unwrap(),
            &provider_principal,
        )
    })
    .unwrap();

    IssueEmrResponse::from(response)
}

#[ic_cdk::query(composite = true, guard = "only_authorized_metrics_collector")]
async fn ping() -> PingResult {
    let emr_registry = with_state(|s| s.config.get().emr_registry());
    let emr_registry_status = emr_registry.ping().await.is_ok();

    let patient_registry = with_state(|s| s.config.get().patient_registry());
    let patient_registry_status = patient_registry.ping().await.is_ok();

    PingResult {
        emr_registry_status,
        patient_registry_status,
    }
}

#[ic_cdk::update]
async fn register_new_provider(req: RegisternewProviderRequest) -> RegisternewProviderResponse {
    let id = with_id_generator_mut(|g| g.generate_id());

    with_state_mut(|s| {
        s.providers
            .register_new_provider(req.provider_principal, req.display_name, req.address, id)
    })
    .unwrap();

    RegisternewProviderResponse {}
}

/// is_valid_provider
/// To check if the caller principal is a valid provider
#[ic_cdk::query]
fn is_valid_provider(req: Principal) -> bool {
    with_state(|s| s.providers.is_valid_provider(&req))
}

#[ic_cdk::update(guard = "only_provider")]
async fn update_emr(req: crate::api::UpdateEmrRequest) -> crate::api::UpdateEmrResponse {
    let emr_registry = with_state(|s| s.config.get().emr_registry());
    let patient_registry = with_state(|s| s.config.get().patient_registry());

    ProviderRegistry::do_call_update_emr(req, emr_registry, patient_registry).await;

    crate::api::UpdateEmrResponse {}
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn update_emr_registry_principal(req: UpdateEmrRegistryRequest) {
    with_state_mut(|s| {
        let mut config = s.config.get().to_owned();

        config.update_default_emr_registry_principal(req.principal);

        match s.config.set(config) {
            Ok(_) => (),
            Err(e) => ic_cdk::trap(&format!("failed to update emr registry principal: {:?}", e)),
        }
    })
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn update_patient_registry_principal(req: UpdatePatientRegistryRequest) {
    with_state_mut(|s| {
        let mut config = s.config.get().to_owned();

        config.update_patient_registry_principal(req.principal);

        match s.config.set(config) {
            Ok(_) => (),
            Err(e) => ic_cdk::trap(&format!(
                "failed to update patient registry principal: {:?}",
                e
            )),
        }
    })
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn suspend_provider(req: SuspendRequest) {
    with_state_mut(|s| s.providers.suspend_provider(req.principal)).unwrap()
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn unsuspend_provider(req: UnSuspendRequest) {
    with_state_mut(|s| s.providers.unsuspend_provider(&req.principal)).unwrap()
}

#[ic_cdk::query]
fn get_provider_info_with_principal(req: ProviderInfoRequest) -> ProviderInfoResponse {
    req.provider
        .into_iter()
        .map(|principal| {
            with_state(|s| {
                s.providers
                    .provider_info_with_principal(&principal)
                    .unwrap()
            })
        })
        .collect::<Vec<_>>()
        .into()
}

#[ic_cdk::query]
fn get_provider_batch(req: GetProviderBatchRequest) -> GetProviderBatchResponse {
    req.ids
        .into_iter()
        .map(|id| with_state(|s| s.providers.provider_info_with_internal_id(&id).unwrap()))
        .collect::<Vec<_>>()
        .into()
}

#[ic_cdk::query(guard = "only_canister_owner")]
fn get_provider_list(req: GetProviderListRequest) -> GetProviderListResponse {
    let providers = with_state(|s| s.providers.get_all_providers(req.page, req.limit).unwrap());
    GetProviderListResponse::from(providers)
}

ic_cdk::export_candid!();
