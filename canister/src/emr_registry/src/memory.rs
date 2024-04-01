use canister_common::generate_memory_id;

use crate::{config::CanisterConfig, registry::CoreEmrRegistry};

generate_memory_id!(CoreEmrRegistry, CanisterConfig);
