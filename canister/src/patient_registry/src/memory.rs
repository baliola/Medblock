use canister_common::generate_memory_id;

use crate::{config::CanisterConfig, registry::{ EmrBindingMap, OwnerMap }};

generate_memory_id!(EmrBindingMap, OwnerMap, CanisterConfig);
