use ic_stable_structures::{BTreeMap, Log};

use crate::internal_types::{AsciiRecordsKey, Id};

use super::ModifyEmr;

type UserId = Id;
type ProviderId = Id;
type EmrId = Id;
type RecordsKey = AsciiRecordsKey;
type OpaqueEmrValue = Vec<u8>;

type CompositeKey = (UserId, ProviderId, EmrId, RecordsKey);

pub struct CoreRegistry(BTreeMap<CompositeKey, OpaqueEmrValue>);

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

    pub fn get_list_provider_batch(page: u64, limit: u64) -> Vec<EmrId> {
        todo!()
    }

    pub fn get_list_user_batch(page: u64, limit: u64) -> Vec<EmrId> {
        todo!()
    }

    pub fn get_record_id() -> /** TODO: make type for conversing from opaque emr key-value type to type that can be seriaized */ () {
        todo!()
    }
}


// TODO: implement log for core registry
pub struct ActivityLog(/** TODO */);