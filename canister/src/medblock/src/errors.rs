//! this is an canister overarching error that is meant to be used to display errors to the user

#[derive(thiserror::Error, Debug, candid::CandidType, serde::Deserialize)]
pub enum CanisterError {
    
}