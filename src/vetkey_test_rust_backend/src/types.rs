use std::str::FromStr;

use candid::Principal;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::{mem::Memory, wrapper::Bounded};

macro_rules! native_bounded {
    ($($ident:ty: $ty:ty;)*) => {
        $(
            impl Bounded for $ident {
                const BOUND: Bound = <$ty as Storable>::BOUND;
            }
        )*
    };
}

#[derive(Serialize, Deserialize)]
pub struct IcPrincipal(pub String);

native_bounded! {
    IcPrincipal: String;
}

impl TryFrom<String> for IcPrincipal {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Principal::from_str(&value) {
            Ok(_) => Ok(Self(value)),
            Err(e) => Err(e.into()),
        }
    }
}

pub type VerifiedEmrManagerSet = ic_stable_structures::BTreeMap<IcPrincipal, (), Memory>;
