use canister_common::generate_memory_id;

use crate::{
    config::CanisterConfig,
    consent::{InnerConsentMap, ProviderConsentSet, SessionMap},
    log::{ActivityEntryMemory, ActivityIndexMemory, LogMapIndex},
    registry::{
        AdminMap, EmrBindingMap, GroupConsentMap, GroupMap, HeaderStatusMap, InfoMap,
        InnerGroupConsentMap, OwnerMap,
    },
};

pub struct UpgradeMemory;
generate_memory_id!(
    UpgradeMemory,
    EmrBindingMap,
    OwnerMap,
    AdminMap,
    GroupMap,
    GroupConsentMap,
    InnerGroupConsentMap,
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
