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
    authorized_callers: Vec<Principal>,
    authorized_metrics_collectors: Vec<Principal>,
}

metrics!(CanisterConfig: AuthorizedCallers);

impl Metrics<AuthorizedCallers> for CanisterConfig {
    fn metrics_name() -> &'static str {
        "authorized_callers"
    }

    fn metrics_measurements() -> &'static str {
        ""
    }

    fn update_measurements(&self) {
        // no-op
    }

    fn get_measurements(&self) -> String {
        self.authorized_callers
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", ")
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
            authorized_callers: vec![],
            authorized_metrics_collectors: vec![],
        }
    }
}

impl CanisterConfig {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn is_authorized_caller(&self, caller: &Principal) -> bool {
        self.authorized_callers.contains(caller)
    }

    pub fn add_authorized_caller(&mut self, caller: Principal) {
        if self.authorized_callers.contains(&caller) {
            return;
        }

        self.authorized_callers.push(caller);
    }

    pub fn remove_authorized_caller(&mut self, caller: Principal) {
        self.authorized_callers.retain(|c| c != &caller);
    }

    pub fn get_authorized_callers(&self) -> Vec<Principal> {
        self.authorized_callers.clone()
    }

    pub fn get_authorized_metrics_collectors(&self) -> Vec<Principal> {
        self.authorized_metrics_collectors.clone()
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
