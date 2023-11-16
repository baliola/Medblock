use std::{mem::size_of, str::FromStr};

use candid::Principal;
use ic_stable_structures::{storable::Bound, BTreeMap, Storable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    mem::Memory,
    wrapper::{Bounded, Stable},
};

/// auto implement [Bounded] for types that have same size as primitives types
///
/// useful for implementing [Bounded] for newtypes.
macro_rules! native_bounded {
    ($($ident:ty: $ty:ty;)*) => {
        $(
            impl Bounded for $ident {
                const BOUND: Bound = <$ty as Storable>::BOUND;
            }
        )*
    };
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IcPrincipal(pub String);

impl From<IcPrincipal> for Principal {
    fn from(value: IcPrincipal) -> Self {
        Principal::from_str(&value.0).expect("should be a valid principal")
    }
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

pub struct VerifiedEmrManagerSet(pub BTreeMap<Stable<IcPrincipal>, (), Memory>);

#[derive(Serialize, Deserialize)]
pub struct EmrId(pub Uuid);

impl TryFrom<String> for EmrId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Uuid::parse_str(&value) {
            Ok(u) => Ok(Self(u)),
            Err(e) => Err(e.into()),
        }
    }
}

pub type EmrMetadataKey = String;
pub type EmrMetadataValue = String;
pub type EmrStorageMap = BTreeMap<(Stable<EmrId>, EmrMetadataKey), EmrMetadataValue, Memory>;

native_bounded! {
    IcPrincipal: String;
    EmrId: u16;
}
