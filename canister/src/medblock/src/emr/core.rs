use ic_stable_structures::{ storable::Bound, BTreeMap };
use parity_scale_codec::{ Decode, Encode };

use crate::{
    impl_max_size,
    internal_types::{ AsciiRecordsKey, Id },
    mem::shared::{ MemBoundMarker, Memory, Stable },
};



type UserId = Id;
type ProviderId = Id;
type EmrId = Id;
type RecordsKey = AsciiRecordsKey;
type ArbitraryEmrValue = String;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub struct CompositeKey(UserId, ProviderId, EmrId, RecordsKey);
impl_max_size!(CompositeKey, UserId, ProviderId, EmrId, RecordsKey);

impl MemBoundMarker for CompositeKey {
    const BOUND: Bound = Bound::Bounded { max_size: Self::max_size() as u32, is_fixed_size: false };
}

pub struct CoreRegistry(BTreeMap<Stable<CompositeKey>, ArbitraryEmrValue, Memory>);

impl CoreRegistry {
    pub fn add() {
        todo!()
    }

    pub fn update() {
        todo!()
    }

    pub fn update_batch() {
        todo!()
    }

    pub fn remove_record() {
        todo!()
    }

    pub fn get_list_provider_batch(_page: u64, _limit: u64) -> Vec<EmrId> {
        todo!()
    }

    pub fn get_list_user_batch(_page: u64, _limit: u64) -> Vec<EmrId> {
        todo!()
    }

    /** TODO: make type for conversing from opaque emr key-value type to type that can be seriaized */
    pub fn get_record_id() {
        todo!()
    }
}

// // TODO: implement log for core registry
// pub struct ActivityLog(/** TODO */);
