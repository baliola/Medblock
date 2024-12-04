use canister_common::generate_memory_id;

use crate::{
    config::CanisterConfig,
    consent::{ InnerConsentMap, ProviderConsentSet, SessionMap },
    log::{ ActivityEntryMemory, ActivityIndexMemory, LogMapIndex },
    registry::{ EmrBindingMap, HeaderStatusMap, InfoMap, OwnerMap, AdminMap, GroupMap },
};

pub struct UpgradeMemory;
generate_memory_id!(
    UpgradeMemory,
    EmrBindingMap,
    OwnerMap,
    AdminMap,
    GroupMap,
    CanisterConfig,
    InfoMap,
    HeaderStatusMap,
    ProviderConsentSet,
    ActivityEntryMemory,
    ActivityIndexMemory,
    LogMapIndex,
    InnerConsentMap,
    SessionMap
);
