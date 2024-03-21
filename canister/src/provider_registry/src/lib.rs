use std::{ borrow::Borrow, cell::RefCell, time::Duration };

use api::{ IssueEmrResponse, PingResult };
use canister_common::{
    common::freeze::FreezeThreshold,
    id_generator::IdGenerator,
    mmgr::MemoryManager,
    random::{ CallError, CanisterRandomSource },
    statistics::{ self, traits::OpaqueMetrics },
};
use declarations::emr_registry::{ self, emr_registry };
use ic_principal::Principal;
use registry::ProviderRegistry;

mod declarations;
mod registry;
mod config;
mod types;
pub mod api;

/// TODO: benchmark this
const CANISTER_CYCLE_THRESHOLD: u128 = 300_000;

pub struct State {
    providers: ProviderRegistry,
    config: config::CanisterConfig,
    memory_manager: MemoryManager,
    freeze_threshold: FreezeThreshold,
}

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::new(None);
    static ID_GENERATOR: RefCell<Option<IdGenerator<CanisterRandomSource>>> = RefCell::new(None);
}

/// A helper method to read the state.
///
/// Precondition: the state is already initialized.
pub fn with_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(cell.borrow().as_ref().expect("state not initialized")))
}

/// A helper method to mutate the state.
///
/// Precondition: the state is already initialized.
pub fn with_state_mut<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(cell.borrow_mut().as_mut().expect("state not initialized")))
}

#[ic_cdk::inspect_message]
fn inspect_message() {
    verified_caller().expect("caller is not verified");
    with_state(|s| s.freeze_threshold.check());

    ic_cdk::api::call::accept_message()
}

fn verified_caller() -> Result<Principal, String> {
    let caller = ic_cdk::caller();

    ic_cdk::eprintln!("caller : {}", caller);

    if caller.ne(&Principal::anonymous()) {
        return Err(String::from("anonymous caller is not allowed"));
    }
    Ok(caller)
}

// guard function
fn only_canister_owner() -> Result<(), String> {
    return Ok(());
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let caller = verified_caller()?;

        if !state.config.is_canister_owner(&caller) {
            return Err("only canister owner can call this method".to_string());
        }

        Ok(())
    })
}

// guard function
fn only_provider() -> Result<(), String> {
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let caller = verified_caller()?;

        if !state.providers.is_valid_provider(&caller) {
            return Err("only provider can call this method".to_string());
        }

        // safe to unwrap as we already check if caller is a valid provider or not
        if state.providers.is_provider_suspended(&caller).unwrap() {
            return Err("provider is suspended".to_string());
        }

        Ok(())
    })
}

fn initialize_id_generator() {
    ic_cdk_timers::set_timer(Duration::from_secs(3), || {
        ic_cdk::spawn(async move {
            let rng = CanisterRandomSource::new().await;
            let id_generator = IdGenerator::new(rng);

            ID_GENERATOR.with(|id_gen| {
                *id_gen.borrow_mut() = Some(id_generator);
            });

            ic_cdk::print("id generator initialized");
        })
    });
}

#[ic_cdk::init]
fn canister_init() {
    STATE.with(move |state| {
        let memory_manager = MemoryManager::new();

        let init = State {
            providers: ProviderRegistry::new(&memory_manager),
            config: config::CanisterConfig::default(),
            memory_manager,
            freeze_threshold: FreezeThreshold::new(CANISTER_CYCLE_THRESHOLD),
        };

        *state.borrow_mut() = Some(init);

        ic_cdk::print("state initialized");
    });

    initialize_id_generator()
}

#[ic_cdk::query]
fn metrics() -> String {
    with_state(|s| {
        [
            s.providers.measure(),
            statistics::canister::BlockchainMetrics::measure(),
            statistics::canister::MemoryStatistics::measure(),
        ].join("\n")
    })
}

#[ic_cdk::query(guard = "only_provider")]
fn emr_list_provider(req: types::EmrListProviderRequest) -> types::EmrListProviderResponse {
    with_state(|state| {
        let provider = verified_caller().unwrap();

        let limit = state.config.max_item_per_response().min(req.limit);

        state.providers
            .get_issued(&provider, req.page, limit as u64)
            .unwrap()
            .into()
    })
}

#[ic_cdk::update(guard = "only_provider")]
async fn issue_emr(req: api::IssueEmrRequest) -> api::IssueEmrResponse {
    let args = with_state(|s| s.providers.build_args_call_emr_canister(req)).unwrap();

    // safe to unwrap as the provider id comes from canister
    let provider_principal = Principal::from_text(args.provider_id.clone()).unwrap();

    let response = ProviderRegistry::issue_call_create_emr(args).await;

    with_state_mut(|s|
        s.providers.issue_emr(
            response.header.emr_id.clone().try_into().unwrap(),
            &provider_principal
        )
    ).unwrap();

    IssueEmrResponse::from(response)
}

#[ic_cdk::query(composite = true)]
async fn ping() -> PingResult {
    let emr_registry_status = match emr_registry.ping().await {
        Ok(_) => true,
        Err(_) => false,
    };

    // let patient_registry_status = api::ping().await;

    PingResult {
        emr_registry_status,
        patient_registry_status: false,
    }
}

ic_cdk::export_candid!();
