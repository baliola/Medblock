//! this is an canister overarching error that is meant to be used to display errors to the user

#[derive(thiserror::Error, Debug, candid::CandidType, serde::Deserialize)]
pub enum CanisterError {
    #[error(transparent)] EmrRegistryError(#[from] crate::emr::RegistryError),
    #[error(transparent)] ProviderRegistryError(#[from] crate::emr::providers::RegistryError),
}
