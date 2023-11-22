//! An example showcasing how stable structures, and specifically StableBTreeMap, can be used in
//! production alongside other state that is stored on the heap and is serialized/deserialized on
//! every upgrade.
//!
//! This example showcases how you can include stable structures in your projects. For simpler
//! examples, checkout the other examples in the `examples` directory.
mod mem;
mod wrapper;
mod types;
mod encryption;
pub mod index;
mod macros;

use ic_stable_structures::{StableBTreeMap};
use mem::{Memory};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

// The state of the canister.
#[derive(Serialize, Deserialize)]
struct State {
    // Data that lives on the heap.
    // This is an example for data that would need to be serialized/deserialized
    // on every upgrade for it to be persisted.
    data_on_the_heap: Vec<u8>,

    // An example `StableBTreeMap`. Data stored in `StableBTreeMap` doesn't need to
    // be serialized/deserialized in upgrades, so we tell serde to skip it.
    #[serde(skip, default = "init_stable_data")]
    stable_data: StableBTreeMap<u128, u128, Memory>,
}
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}



fn init_stable_data() -> StableBTreeMap<u128, u128, Memory> {
    StableBTreeMap::init(crate::mem::MemoryMetadata::get_stable_btree_memory())
}

impl Default for State {
    fn default() -> Self {
        Self {
            data_on_the_heap: vec![],
            stable_data: init_stable_data(),
        }
    }
}
