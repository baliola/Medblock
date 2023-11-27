use std::marker::PhantomData;

use candid::Principal;
use ic_stable_structures::storable::Bound;
use serde::{Deserialize, Serialize};

use crate::{
    types::Set,
    wrapper::{Bounded, Stable},
};

use self::role::{Patient, Provider, RoleMarker};

pub mod role {
    pub trait RoleMarker {}
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Patient;

    impl RoleMarker for Patient {}

    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Provider;

    impl RoleMarker for Provider {}
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct User<Role: RoleMarker> {
    principal: Principal,
    role: PhantomData<Role>,
}

impl<Role: RoleMarker> Bounded for User<Role> {
    // 30 because we effectively only store 30 bytes data in the principal
    const BOUND: Bound = Bound::Bounded {
        max_size: 30,
        is_fixed_size: true,
    };
}

impl<Role: RoleMarker> User<Role> {
    pub fn new(principal: Principal) -> Self {
        Self {
            principal,
            role: PhantomData,
        }
    }
}

pub struct Supervisor {
    users: Set<Stable<User<Patient>>>,
    providers: Set<Stable<User<Provider>>>,
}
