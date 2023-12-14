use candid::Principal;

pub struct CanisterConfig {
    owner: Principal,
}

impl CanisterConfig {
    pub fn new(owner: Principal) -> Self {
        Self { owner }
    }

    pub fn is_canister_owner(&self, principal: &Principal) -> bool {
        self.owner.eq(principal)
    }
}
