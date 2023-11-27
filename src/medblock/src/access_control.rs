use std::marker::PhantomData;

use candid::Principal;

use self::role::RoleMarker;

pub mod role {
    pub trait RoleMarker {}
    pub struct Patient;

    impl RoleMarker for Patient {}

    pub struct Provider;

    impl RoleMarker for Provider {}
}

pub struct User<Role: RoleMarker> {
    principal: Principal,
    role: PhantomData<Role>,
}
