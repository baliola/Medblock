use canister_common::generate_memory_id;

use crate::{ config::CanisterConfig, registry::CoreEmrRegistry };

pub struct UpgradeMemory;
generate_memory_id!(UpgradeMemory, CoreEmrRegistry, CanisterConfig);
