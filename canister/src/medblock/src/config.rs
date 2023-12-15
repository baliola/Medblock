use candid::Principal;

pub struct CanisterConfig {
    owner: Principal,
    // TODO: make this configurable
    max_item_per_response: usize,
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
        }
    }
}

impl CanisterConfig {
    /// constant values to implement pagination,
    /// this values will be used to limit the number of emrs returned. to account for 2MB response limit.
    ///
    /// initially set to 10.
    const INITIAL_MAX_EMR_RESPONSE: usize = 10;

    pub fn new(owner: Principal) -> Self {
        Self {
            owner,
            ..Default::default()
        }
    }

    pub fn is_canister_owner(&self, principal: &Principal) -> bool {
        self.owner.eq(principal)
    }
}
