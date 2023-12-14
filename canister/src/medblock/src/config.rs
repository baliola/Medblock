use candid::Principal;

pub struct CanisterConfig {
    owner: Principal,
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
            owner: ic_cdk::caller(),
        }
    }
}

impl CanisterConfig {
    pub fn new(owner: Principal) -> Self {
        Self { owner }
    }

    pub fn is_canister_owner(&self, principal: &Principal) -> bool {
        self.owner.eq(principal)
    }
}
