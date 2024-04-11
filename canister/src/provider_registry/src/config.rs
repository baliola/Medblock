use candid::{ CandidType, Principal };
use canister_common::{
    impl_max_size,
    impl_mem_bound,
    metrics,
    mmgr::MemoryManager,
    stable::{ Candid, Memory, Stable, ToStable },
    statistics::traits::Metrics,
};
use ic_stable_structures::Cell;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterConfig {
    owner: Principal,
    // TODO: make this configurable
    max_item_per_response: u8,

    default_emr_registry: Principal,
    patient_registry: Principal,
    emr_registries: Vec<Principal>,
    authorized_metrics_collectors: Vec<Principal>,
}

metrics!(CanisterConfig: EmrRegistry,PatientRegistry,MetricsCollector);
impl Metrics<EmrRegistry> for CanisterConfig {
    fn metrics_name() -> &'static str {
        "emr_registry"
    }

    fn metrics_measurements() -> &'static str {
        ""
    }

    fn update_measurements(&self) {}

    fn get_measurements(&self) -> String {
        self.emr_registries
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl Metrics<PatientRegistry> for CanisterConfig {
    fn metrics_name() -> &'static str {
        "patient_registry"
    }

    fn metrics_measurements() -> &'static str {
        ""
    }

    fn update_measurements(&self) {}

    fn get_measurements(&self) -> String {
        self.patient_registry.to_string()
    }
}

impl Metrics<MetricsCollector> for CanisterConfig {
    fn metrics_name() -> &'static str {
        "metrics_collector"
    }

    fn metrics_measurements() -> &'static str {
        ""
    }

    fn update_measurements(&self) {}

    fn get_measurements(&self) -> String {
        self.authorized_metrics_collectors
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl_max_size!(for CanisterConfig: Principal, u8);
impl_mem_bound!(for CanisterConfig: bounded; fixed_size: false);

impl CanisterConfig {
    pub fn init(memory_manager: &MemoryManager) -> Cell<Stable<CanisterConfig, Candid>, Memory> {
        // safe as we're using layout version 1
        memory_manager
            .get_memory::<_, Self>(|m| Cell::init(m, CanisterConfig::default().to_stable()))
            .unwrap()
    }
}

impl Default for CanisterConfig {
    /// Returns a new instance of `CanisterConfig` with default values.
    /// will use current caller as the owner.
    ///
    /// # Panics
    /// will panic if called outside canister execution environment. don't call this in test,
    /// use `CanisterConfig::new` instead.
    fn default() -> Self {
        Self {
            max_item_per_response: Self::INITIAL_MAX_EMR_RESPONSE,
            owner: ic_cdk::caller(),
            default_emr_registry: Principal::anonymous(),
            patient_registry: Principal::anonymous(),
            emr_registries: vec![Principal::anonymous()],
            authorized_metrics_collectors: vec![],
        }
    }
}

impl CanisterConfig {
    /// constant values to implement pagination,
    /// this values will be used to limit the number of emrs returned. to account for 2MB response limit.
    ///
    /// initially set to 10.
    const INITIAL_MAX_EMR_RESPONSE: u8 = 10;

    pub fn new(owner: Principal) -> Self {
        Self {
            owner,
            ..Default::default()
        }
    }

    pub fn emr_registry(&self) -> crate::declarations::emr_registry::EmrRegistry {
        crate::declarations::emr_registry::EmrRegistry(self.default_emr_registry)
    }

    pub fn patient_registry(&self) -> crate::declarations::patient_registry::PatientRegistry {
        crate::declarations::patient_registry::PatientRegistry(self.patient_registry)
    }

    pub fn update_default_emr_registry_principal(&mut self, principal: Principal) {
        let prev_default = self.default_emr_registry;

        self.default_emr_registry = principal;

        self.emr_registries.iter_mut().for_each(|emr_registry| {
            if *emr_registry == prev_default {
                *emr_registry = principal;
            }
        });
    }

    pub fn update_patient_registry_principal(&mut self, principal: Principal) {
        self.patient_registry = principal;
    }

    pub fn is_canister_owner(&self, principal: &Principal) -> bool {
        self.owner.eq(principal)
    }

    pub fn max_item_per_response(&self) -> u8 {
        self.max_item_per_response
    }

    pub fn add_authorized_metrics_collector(&mut self, collector: Principal) {
        if self.authorized_metrics_collectors.contains(&collector) {
            return;
        }

        self.authorized_metrics_collectors.push(collector);
    }

    pub fn remove_authorized_metrics_collector(&mut self, collector: Principal) {
        self.authorized_metrics_collectors.retain(|c| c != &collector);
    }

    pub fn is_authorized_metrics_collector(&self, collector: &Principal) -> bool {
        self.authorized_metrics_collectors.contains(collector)
    }
}
