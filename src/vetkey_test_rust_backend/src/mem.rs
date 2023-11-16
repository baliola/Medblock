use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl, Storable};
use paste::paste;
use std::cell::RefCell;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

macro_rules! impl_memory_id {
    // Empty case to end the recursion
    (index $n:expr => ) => {};

    //recursion till none
    (index $lit:expr => $ident:tt $(, $rest:tt)*) => {
        paste! {const[<$ident:upper _MEMORY_ID>]: MemoryId = MemoryId::new($lit);}

        paste! {pub fn [<get_ $ident:lower _memory>]() -> Memory {
            MEMORY_MANAGER.with(|m| m.borrow().get(Self::[<$ident:upper _MEMORY_ID>]))
        }}

        impl_memory_id!(index $lit + 1 => $($rest),*);
    };

    //bootstrap to first recursiong
    ($($ident:tt),*) => {
        impl_memory_id!(index 0 => $($ident),*);
    };
}

pub struct MemoryMetadata;

impl MemoryMetadata {
    impl_memory_id!(UPGRADES, STABLE_BTREE, STABLE_METADATA);
}

thread_local! {
// The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
// return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));    
}
