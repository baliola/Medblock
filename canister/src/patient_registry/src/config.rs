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

    emr_registry: Principal,
}
metrics!(CanisterConfig: Owners, MaxItem);

impl Metrics<Owners> for CanisterConfig {
    fn metrics_name() -> &'static str {
        "canister"
    }

    fn metrics_measurements() -> &'static str {
        "owner"
    }

    fn update_measurements(&self) {
        // no-op
    }

    fn get_measurements(&self) -> String {
        self.owner.to_string()
    }
}

impl Metrics<MaxItem> for CanisterConfig {
    fn metrics_name() -> &'static str {
        "canister"
    }

    fn metrics_measurements() -> &'static str {
        "max_item_per_response"
    }

    fn update_measurements(&self) {
        // no-op
    }

    fn get_measurements(&self) -> String {
        self.max_item_per_response.to_string()
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
            emr_registry: crate::declarations::emr_registry::CANISTER_ID,
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

    pub fn emr_registry(&self) -> crate::declarations::emr_registry::EmrRegistry{
        crate::declarations::emr_registry::EmrRegistry(self.emr_registry)
    }

    pub fn update_emr_registry_principal(&mut self, principal: Principal) {
        self.emr_registry = principal;
    }

    pub fn is_canister_owner(&self, principal: &Principal) -> bool {
        self.owner.eq(principal)
    }

    pub fn max_item_per_response(&self) -> u8 {
        self.max_item_per_response
    }
}
