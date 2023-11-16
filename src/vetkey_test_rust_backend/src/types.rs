use std::{mem::size_of, str::FromStr};

use candid::Principal;
use ic_stable_structures::{storable::Bound, BTreeMap, Storable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    mem::Memory,
    wrapper::{Bounded, Stable},
};
//TODO : find a way to optimize memory usage, especially the key inside the metadata map of the emr

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
pub struct IcPrincipal(String);

impl From<IcPrincipal> for Principal {
    fn from(value: IcPrincipal) -> Self {
        Principal::from_str(&value.0).expect("should be a valid principal")
    }
}

impl TryFrom<String> for IcPrincipal {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok((Principal::from_str(&value).map(|_| Self(value)))?)
    }
}

pub struct VerifiedEmrManagerSet(BTreeMap<Stable<IcPrincipal>, (), Memory>);

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

impl From<EmrId> for String {
    fn from(value: EmrId) -> Self {
        value.0.to_string()
    }
}

pub type EmrMetadataKey = Stable<String>;
// TODO : string for simplicity for now, should find a way to optimize this later.
pub type EmrMetadataValue = Stable<String>;
pub struct EmrStorageMap(BTreeMap<(Stable<EmrId>, EmrMetadataKey), EmrMetadataValue, Memory>);

impl EmrStorageMap {
    const STATIC_EMR_METADATA_KEY: &'static str = "issued_by";

    pub fn insert_emr(
        &mut self,
        emr_id: Stable<EmrId>,
        issued_by: Stable<IcPrincipal>,
        metadata: Vec<(String, String)>,
    ) {
        self.issue(emr_id.clone(), issued_by);
        self.populate_metadata(metadata, emr_id);
    }

    fn populate_metadata(&mut self, metadata: Vec<(String, String)>, emr_id: Stable<EmrId>) {
        for (key, value) in metadata {
            self.0.insert((emr_id.clone(), Stable(key)), Stable(value));
        }
    }

    fn issue(&mut self, emr_id: Stable<EmrId>, issued_by: Stable<IcPrincipal>) {
        self.0.insert(
            (emr_id, Stable(Self::STATIC_EMR_METADATA_KEY.to_string())),
            // clean this later
            issued_by.0 .0.into(),
        );
    }
}

native_bounded! {
    IcPrincipal: String;
    EmrId: u16;
}
