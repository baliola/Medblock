use std::{ cell::RefCell, rc::Rc };

use candid::Principal;
use config::CanisterConfig;
use emr::{
    providers::ProviderRegistry,
    EmrRegistry,
    EmrDisplay,
    FromStableRef,
    patient::NIK,
    RecrodsDisplay,
    Records,
};
use random::{ CanisterRandomSource, CallError };
use types::Id;

use crate::types::UUID_MAX_SOURCE_LEN;

mod config;
mod emr;
mod encryption;
mod log;
mod macros;
mod types;
mod random;

// TODO :  make sure no unwrap() in this canister

#[derive(Default)]
pub struct State {
    emr_registry: EmrRegistry,
    provider_registry: ProviderRegistry,
    config: CanisterConfig,
    rng: Rc<CanisterRandomSource>,
    // TODO : incorporate logs
    // log: Log,
}

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::default();
}

fn verified_caller() -> Result<Principal, String> {
    let caller = ic_cdk::caller();

    ic_cdk::eprintln!("caller : {}", caller);

    if caller.ne(&ic_cdk::export::Principal::anonymous()) {
        return Err(String::from("anonymous caller is not allowed"));
    }
    Ok(caller)
}

fn only_issued_or_owner(emr_id: types::Id) -> Result<(), String> {
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let caller = verified_caller()?;

        // closure for readability
        let is_owner = || state.emr_registry.is_owner_of_emr(&caller, &emr_id);
        let is_issuer = || state.provider_registry.is_issued_by(&caller, &emr_id);

        if !is_owner() && !is_issuer() {
            return Err("only owner or issuer can call this method".to_string());
        }

        Ok(())
    })
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

        if !state.provider_registry.is_valid_provider(&caller) {
            return Err("only provider can call this method".to_string());
        }

        Ok(())
    })
}

// guard function
fn only_patients() -> Result<(), String> {
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let caller = verified_caller()?;

        if !state.emr_registry.is_valid_patient(&caller) {
            return Err("only patient can call this method".to_string());
        }

        Ok(())
    })
}

// guard function
fn only_patients_or_provider() -> Result<(), String> {
    only_patients().or_else(|_| only_provider())
}

async fn generate_id() -> Result<Id, CallError> {
    let rng = STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        state.rng.clone()
    });

    rng.get_random_bytes::<UUID_MAX_SOURCE_LEN>().await.map(|bytes| Id::new(&bytes))
}

#[ic_cdk::init]
fn init() {
    ic_stable_memory::stable_memory_init();

    STATE.with(|state| {
        *state.borrow_mut() = Some(State::default());
    });
}

#[ic_cdk::update(guard = "only_canister_owner")]
#[candid::candid_method(update)]
// TODO : move arguments to a candid struct
async fn register_new_provider(new_provider: Principal, encryted_display_name: String) {
    let id = generate_id().await.unwrap();

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().unwrap();

        state.provider_registry
            .register_new_provider(new_provider, encryted_display_name, id)
            .unwrap()
    })
}

#[ic_cdk::update(guard = "only_canister_owner")]
#[candid::candid_method(update)]
// TODO : move arguments to a candid struct
fn suspend_provider(provider: Principal) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().unwrap();

        state.provider_registry.suspend_provider(provider).unwrap()
    });
}

#[ic_cdk::query(guard = "only_patients_or_provider")]
#[candid::candid_method(query)]
// TODO : move arguments to a candid struct
fn read_emr_by_id(emr_id: types::Id) -> Option<emr::EmrDisplay> {
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let emr = state.emr_registry.get_emr(&emr_id).unwrap();

        Some(EmrDisplay::from_stable_ref(&*emr))
    })
}

#[ic_cdk::update(guard = "only_provider")]
#[candid::candid_method(update)]
async fn create_emr_for_user(owner: NIK, emr_records: RecrodsDisplay) {
    ic_cdk::eprintln!("create_emr_for_user: {}", emr_records.0);

    let records = Records::try_from(emr_records).unwrap();
    let id = generate_id().await.unwrap();

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().unwrap();

        // change the emr version if upgrade happens
        let emr = emr::V001::new(id, records).into();

        let emr_id = state.emr_registry.register_emr(emr, owner).unwrap();

        let caller = verified_caller().unwrap();

        // increment session
        state.provider_registry.issue_emr(&caller, emr_id);
    })
}

#[ic_cdk::query(guard = "only_provider")]
#[candid::candid_method(query)]
// TODO : move arguments to a candid struct
fn emr_list_provider(anchor: u64, max: u8) -> Vec<Id> {
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let provider = verified_caller().unwrap();

        state.provider_registry.get_issued(&provider, anchor, max).unwrap()
    })
}

#[ic_cdk::update(guard = "only_provider")]
#[candid::candid_method(update)]
// TODO : move arguments to a candid struct
fn register_patient(owner: Principal, hashed_nik: NIK) -> Result<(), String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().unwrap();

        state.emr_registry.register_patient(owner, hashed_nik).unwrap();

        Ok(())
    })
}

#[ic_cdk::update(guard = "only_provider")]
#[candid::candid_method(update)]
// TODO : move arguments to a candid struct
fn rebind_patient(owner: Principal, hashed_nik: NIK) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().unwrap();

        state.emr_registry.rebind_patient(owner, hashed_nik).unwrap();
    })
}

#[ic_cdk::update(guard = "only_provider")]
#[candid::candid_method(update)]
// TODO : move arguments to a candid struct
fn revoke_patient_access(owner: Principal) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().unwrap();

        state.emr_registry.revoke_patient_access(&owner);
    })
}

#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    ic_cdk::export::candid::export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir.parent().unwrap().parent().unwrap().join("src").join("medblock");

        let candid = super::export_candid();
        println!("{:?}", candid);
        write(dir.join("medblock.did"), super::export_candid()).expect("Write failed.");
    }
}
