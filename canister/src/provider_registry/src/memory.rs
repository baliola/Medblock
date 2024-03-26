use canister_common::{ generate_memory_id };

use crate::{ config::CanisterConfig, registry::{ Issued, Providers, ProvidersBindings } };

/// needed since the module is imported
pub struct FreezeThresholdMemory;
generate_memory_id!(Providers, ProvidersBindings, Issued, FreezeThresholdMemory, CanisterConfig);
