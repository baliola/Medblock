use canister_common::{ common::{ freeze::FreezeThreshold, Get }, generate_memory_id };

use crate::{ config::CanisterConfig, registry::{ Issued, Providers, ProvidersBindings } };

/// needed since the module is imported
pub struct FreezeThresholdMemory;
generate_memory_id!(Providers, ProvidersBindings, Issued, FreezeThresholdMemory, CanisterConfig);
