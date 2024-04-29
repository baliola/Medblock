use api::{
    AuthorizedCallerRequest,
    CreateEmrRequest,
    CreateEmrResponse,
    ReadEmrByIdRequest,
    ReadEmrByIdResponse,
    RemoveEmrRequest,
    RemoveEmrResponse,
    UpdateEmrRequest,
    UpdateEmrResponse,
};
use candid::{ Decode, Encode };
use canister_common::{
    common::{ self, guard::verified_caller, Get },
    id_generator::IdGenerator,
    log,
    mmgr::MemoryManager,
    opaque_metrics,
    random::CanisterRandomSource,
    register_log,
    stable::{ Candid, Memory, Stable },
    statistics::{ self, traits::OpaqueMetrics },
};
use canistergeek_ic_rust::{ api_type::UpdateInformationRequest, monitor::data_type::DayData };
use config::CanisterConfig;
use ic_cdk::{ init, query, update };
use ic_stable_structures::{ Cell };
use memory::UpgradeMemory;
use std::{ cell::RefCell, io::Write };
use core::time::Duration;

mod key;
mod registry;
pub mod api;
pub mod header;
mod memory;
mod config;

type State = common::State<
    registry::CoreEmrRegistry,
    Cell<Stable<CanisterConfig, Candid>, Memory>,
    ()
>;
register_log!("emr");

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
    static ID_GENERATOR: RefCell<Option<IdGenerator<CanisterRandomSource>>> = const {
        RefCell::new(None)
    };
}
// change this if you want to change the interval of the metrics collection
const METRICS_INTERVAL: Duration = Duration::from_secs(60 * 5); // 5 minutes

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
    ID_GENERATOR.with(|cell| f(cell.borrow_mut().as_mut().expect("id generator not initialized")))
}

/// A helper method to mutate the state.
///
/// Precondition: the state is already initialized.
pub fn with_state_mut<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(cell.borrow_mut().as_mut().expect("state not initialized")))
}

// TODO : add init method

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
fn init_state() -> self::State {
    let memory_manager = MemoryManager::init();

    State::new(
        registry::CoreEmrRegistry::init(&memory_manager),
        CanisterConfig::init(&memory_manager),
        (),
        memory_manager
    )
}

// guard function
fn only_canister_owner() -> Result<(), String> {
    let caller = verified_caller()?;

    match ic_cdk::api::is_controller(&caller) {
        true => Ok(()),
        false => Err("only canister controller can call this method".to_string()),
    }
}

// guard function
fn only_authorized_caller() -> Result<(), String> {
    let caller = verified_caller()?;

    with_state(|s| {
        if !s.config.get().is_authorized_caller(&caller) {
            return Err("only authorized caller can call this method".to_string());
        }

        Ok(())
    })
}

// guard function
fn only_authorized_metrics_collector() -> Result<(), String> {
    let caller = verified_caller()?;

    with_state(|s| {
        if !s.config.get().is_authorized_metrics_collector(&caller) {
            return Err("only authorized metrics collector can call this method".to_string());
        }

        Ok(())
    })
}

fn initialize() {
    let state = init_state();
    STATE.replace(Some(state));
    log!("state initialized");
    initialize_id_generator();
    start_collect_metrics_job();
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
            log!("encoded canister metrics length written successfully : {} bytes", encoded_len);
        }

        Err(e) => {
            log!("OOM ERROR: failed to write encoded canister metrics length {:?}", e);
        }
    }

    let write = writer.write(&encoded);
    match write {
        Ok(_) => {
            log!("encoded canister metrics written");
        }
        Err(e) => {
            log!("OOM ERROR: failed to write encoded canister metrics {:?}", e);
        }
    }
}

fn start_collect_metrics_job() {
    ic_cdk_timers::set_timer_interval(METRICS_INTERVAL, || {
        log!("updating metrics");

        canistergeek_ic_rust::update_information(UpdateInformationRequest {
            metrics: Some(canistergeek_ic_rust::api_type::CollectMetricsRequestType::force),
        });

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
    log!("readed encoded canister metrics length: {:?} bytes", read_len);

    let (monitor_stable_data, logger_stable_data) =
        Decode!(&state_buf, (u8, std::collections::BTreeMap<u32, DayData>), (u8, canistergeek_ic_rust::logger::LogMessageStorage)).unwrap();

    canistergeek_ic_rust::monitor::post_upgrade_stable_data(monitor_stable_data);
    canistergeek_ic_rust::logger::post_upgrade_stable_data(logger_stable_data);
}

#[query(guard = "only_authorized_metrics_collector", name = "getCanistergeekInformation")]
pub async fn canister_geek_metrics(
    request: canistergeek_ic_rust::api_type::GetInformationRequest
) -> canistergeek_ic_rust::api_type::GetInformationResponse<'static> {
    canistergeek_ic_rust::get_information(request)
}

#[update(guard = "only_authorized_metrics_collector", name = "updateCanistergeekInformation")]
pub async fn update_canistergeek_information(
    request: canistergeek_ic_rust::api_type::UpdateInformationRequest
) {
    canistergeek_ic_rust::update_information(request);
}

#[ic_cdk::inspect_message]
fn inspect_message() {
    verified_caller().expect("caller is not verified");

    match only_canister_owner().is_ok() || only_authorized_caller().is_ok() {
        true => ic_cdk::api::call::accept_message(),
        false => ic_cdk::trap("unauthorized"),
    };
}

#[init]
fn init() {
    initialize();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    initialize();
    deserialize_canister_metrics();
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn add_authorized_caller(req: AuthorizedCallerRequest) {
    with_state_mut(|s| {
        let mut config = s.config.get().to_owned();

        config.add_authorized_caller(req.caller);

        s.config.set(config);
    });
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn remove_authorized_caller(req: AuthorizedCallerRequest) {
    with_state_mut(|s| {
        let mut config = s.config.get().to_owned();

        config.remove_authorized_caller(req.caller);

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

#[ic_cdk::update(guard = "only_canister_owner")]
fn remove_authorized_metrics_collector(req: AuthorizedCallerRequest) {
    with_state_mut(|s| {
        let mut config = s.config.get().to_owned();

        config.remove_authorized_metrics_collector(req.caller);

        s.config.set(config);
    });
}

// TODO : add init state
#[ic_cdk::query(guard = "only_authorized_caller")]
fn read_emr_by_id(req: ReadEmrByIdRequest) -> ReadEmrByIdResponse {
    with_state(|s| { s.registry.read_by_id(req.to_read_key()).unwrap().into() })
}

#[ic_cdk::update(guard = "only_authorized_caller")]
fn create_emr(req: CreateEmrRequest) -> CreateEmrResponse {
    let (key, emr) = req.to_args();
    log!("creating emr");

    with_state_mut(|s| s.registry.add(key, emr))
        .unwrap()
        .into()
}

#[ic_cdk::update(guard = "only_authorized_caller")]
fn update_emr(req: UpdateEmrRequest) -> UpdateEmrResponse {
    with_state_mut(|s|
        s.registry.update_batch(req.header.to_partial_update_key(), req.fields).unwrap().into()
    )
}

#[ic_cdk::update(guard = "only_authorized_caller")]
fn remove_emr(req: RemoveEmrRequest) -> RemoveEmrResponse {
    with_state_mut(|s|
        s.registry
            .remove_record(req.header.to_emr_key())
            .map(|_| RemoveEmrResponse::new(true))
            .unwrap()
    )
}

// this will serve as an synchronization function in the future, for now it's only for testing inter-canister calls successfully
#[ic_cdk::query(guard = "only_authorized_caller")]
fn ping() {
    // no-op
}

#[ic_cdk::query(guard = "only_authorized_metrics_collector")]
fn metrics() -> String {
    with_state(|s| {
        [
            opaque_metrics!(s.registry),
            statistics::canister::BlockchainMetrics::measure(),
            statistics::canister::MemoryStatistics::measure(),
            OpaqueMetrics::measure(s.config.get().as_ref()),
        ].join("\n")
    })
}

ic_cdk::export_candid!();
