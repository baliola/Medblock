use canister_common::{ generate_memory_id };

use crate::{ config::CanisterConfig, registry::{ Issued, Providers, ProvidersBindings } };

/// needed since the module is imported
pub struct FreezeThresholdMemory;
pub struct UpgradeMemory;
generate_memory_id!(
    UpgradeMemory,
    Providers,
    ProvidersBindings,
    Issued,
    FreezeThresholdMemory,
    CanisterConfig
);
