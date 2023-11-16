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

/// High-level wrapper and presentation for emr
/// this the type that should be returned as the return type of the canister public api
#[derive(Clone, Serialize, PartialEq, Eq)]
pub struct Emr {
    id: Stable<EmrId>,
    issued_by: Stable<IcPrincipal>,
    metadata: Vec<(EmrMetadataKey, EmrMetadataValue)>,
}

impl Emr {
    fn random_id() -> EmrId {
        EmrId(Uuid::new_v4())
    }

    pub fn new(issued_by: IcPrincipal, metadata: Vec<(String, String)>) -> Self {
        Self {
            id: Self::random_id().into(),
            issued_by: Stable(issued_by),
            metadata: metadata
                .into_iter()
                .map(|(k, v)| (Stable(k), Stable(v)))
                .collect(),
        }
    }

    /// find metadata by key
    pub fn find(&self, k: &str) -> Option<&str> {
        self.metadata
            .iter()
            .find(|(_k, v)| _k.0.eq(k))
            .map(|(_k, v)| v.as_str())
    }

    /// add metadata to the emr
    pub fn add_metadata(&mut self, k: String, v: String) {
        self.metadata.push((Stable(k), Stable(v)));
    }

    /// replace metadata by key
    pub fn replace_metadata(&mut self, k: String, v: String) {
        self.metadata
            .iter_mut()
            .find(|(key, _)| key.0 == k)
            .map(|(_, value)| *value = Stable(v));
    }

    /// remove metadata by key
    /// return true if the metadata was found and removed
    pub fn remove_metadata(&mut self, k: String) -> bool {
        let index = self
            .metadata
            .iter()
            .enumerate()
            .find(|(_, (key, _))| key.0 == k)
            .map(|(index, _)| index);

        if let Some(index) = index {
            self.metadata.remove(index);
            true
        } else {
            false
        }
    }
}

impl EmrStorageMap {
    const STATIC_EMR_METADATA_KEY: &'static str = "issued_by";

    pub fn insert_emr(&mut self, emr: Emr) {
        self.issue(emr.id.clone(), emr.issued_by);
        self.populate_metadata(emr.metadata, emr.id);
    }

    fn populate_metadata(
        &mut self,
        metadata: Vec<(Stable<String>, Stable<String>)>,
        emr_id: Stable<EmrId>,
    ) {
        for (key, value) in metadata {
            self.0.insert((emr_id.clone(), key), value);
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
