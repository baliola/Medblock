use canister_common::generate_memory_id;

use crate::{ config::CanisterConfig, registry::{ EmrBindingMap, HeaderStatusMap, InfoMap, OwnerMap } };

pub struct UpgradeMemory;
generate_memory_id!(UpgradeMemory, EmrBindingMap, OwnerMap, CanisterConfig, InfoMap, HeaderStatusMap);
