use std::{mem::size_of, ops::RangeBounds, str::FromStr};

use candid::Principal;
use ic_stable_structures::{storable::Bound, BTreeMap, Storable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    mem::Memory,
    wrapper::{Bounded, Stable},
};
//TODO : find a way to optimize memory usage, especially the key inside the metadata map of the emr


/// cutting boiler plate for implementing bounded traits on types
macro_rules! bounded {
    (@CONSTRUCT ) => {};


    (@CONSTRUCT $ident:tt:Unbounded; $($rest:tt)*) => {
        impl Bounded for $ident {
            const BOUND: Bound = Bound::Unbounded;
        }

        native_bounded!(@CONSTRUCT $($rest)*);
    };


    (@CONSTRUCT $ident:ident: $ty:ty; $($rest:tt)*) => {
            impl Bounded for $ident {
                const BOUND: Bound = <$ty as Storable>::BOUND;
            }

            native_bounded!(@CONSTRUCT $($rest)*);
    };

    (@CONSTRUCT $ident:ty:{
        max_size: $max:expr,
        is_fixed: $is_fixed:expr,
    }; $($rest:tt)*)=>{
        impl Bounded for $ident {
            const BOUND: Bound = Bound::Bounded{
                max_size: $max,
                is_fixed_size: $is_fixed,

            };
        }

        native_bounded!(@CONSTRUCT $($rest)*);

    };

    ($($ident:tt: $any_expr:tt;)*) => {
        native_bounded!(@CONSTRUCT $($ident: $any_expr;)*);
    };

}

bounded! {
    IcPrincipal: {
        max_size: size_of::<Principal>() as u32,
        is_fixed: true,
    };
    EmrId: u16;
}

/// wrapper types for stable [BtreeMap]
pub type Map<K, V>
where
    K: Storable + Ord + Clone,
    V: Storable,
= BTreeMap<K, V, Memory>;

/// wrapper types for stable [BtreeMap] as set
pub type Set<T>
where
    T: Storable + Ord + Clone,
= BTreeMap<T, (), Memory>;
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IcPrincipal(Principal);

pub struct VerifiedEmrManagerSet(Set<Stable<IcPrincipal>>);

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

pub struct IssuerToEmrMap(Set<(Stable<IcPrincipal>, Stable<EmrId>)>);

impl IssuerToEmrMap {
    pub(self) fn issue(&mut self, from: Stable<IcPrincipal>, id: Stable<EmrId>) {
        self.0.insert((from, id), ());
    }

    pub(self) fn get_all_issued_by(&self, from: Stable<IcPrincipal>) -> Vec<Stable<EmrId>> {
        self.0
            .range(((from.clone()), Stable(EmrId(Uuid::nil()))))
            .filter(|((issuer, _), _)| issuer == &from)
            .map(|((_, id), _)| id.clone())
            .collect()
    }
}

pub type EmrMetadataKey = Stable<String>;
// TODO : string for simplicity for now, should find a way to optimize this later.
pub type EmrMetadataValue = Stable<String>;
pub struct EmrStorageMap(Map<(Stable<EmrId>, EmrMetadataKey), EmrMetadataValue>);

impl EmrStorageMap {
    const STATIC_EMR_METADATA_KEY: &'static str = "issued_by";

    pub(self) fn insert_emr(&mut self, emr: Emr) {
        self.issue(emr.id.clone(), emr.issued_by);
        self.populate_metadata(emr.metadata, emr.id);
    }

    pub(self) fn find_all_with_ids(&self, ids: &[Stable<EmrId>]) -> Option<Vec<Emr>> {
        let mut emrs = Vec::with_capacity(ids.len());

        for id in ids {
            let emr = self.find_by_id(id).unwrap();
            emrs.push(emr);
        }

        Some(emrs)
    }

    pub(self) fn update_at_id() {
        todo!()
    }

    pub(self) fn remove_at_id() {
        todo!()
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

    fn find_by_id(&self, id: &Stable<EmrId>) -> Option<Emr> {
        let Some(issued_by) = self
            .0
            .get(&(
                id.clone(),
                Stable(Self::STATIC_EMR_METADATA_KEY.to_string()),
            ))
            .clone()
        else {
            return None;
        };

        let issued_by = IcPrincipal(issued_by.0);

        let metadata = self
            .0
            .iter()
            .filter(|((emr_id, _), _)| emr_id == id)
            .map(|((_, key), value)| (key.clone(), value.clone()))
            .collect();

        Some(Emr {
            id: id.clone(),
            issued_by: Stable(issued_by),
            metadata,
        })
    }
}
