use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, Storable};
use paste::paste;
use std::cell::RefCell;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

/// max size of metadata key in bytes, eyeballed this. not for production use
const MAX_METADATA_KEY_SIZE_BYTES: u32 = 32;
/// max size of metadata key in bytes, eyeballed this. not for production use
const MAX_METADATA_VALUE_SIZE_BYTES: u32 = 255;

/// macro for cutting boiler plate for storable impls.
/// can only be used for newtypes that its inner value type implements storable
macro_rules! impl_storable {
    ($($ident:ty);*) => {};
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct MetadataKey(String);

impl Storable for MetadataKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // String already implements `Storable`.
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(String::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_METADATA_KEY_SIZE_BYTES,
        is_fixed_size: false,
    };
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct MetadataValue(String);

impl Storable for MetadataValue {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // String already implements `Storable`.
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(String::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_METADATA_VALUE_SIZE_BYTES,
        is_fixed_size: false,
    };
}

macro_rules! impl_memory_id {
    // Empty case to end the recursion
    (index $n:expr => ) => {};

    //recursion till none
    (index $lit:expr => $ident:ident $(, $rest:ident),*) => {
        paste! {const[<$ident:upper _MEMORY_ID>]: MemoryId = MemoryId::new($lit);}

        paste! {pub fn [<get_ $ident:lower _memory>]() -> Memory {
            MEMORY_MANAGER.with(|m| m.borrow().get(Self::[<$ident:upper _MEMORY_ID>]))
        }}

        impl_memory_id!(index $lit + 1 => $($rest),*);
    };

    //bootstrap to first recursiong
    ($($ident:ident),+) => {
        impl_memory_id!(index 0 => $($ident),*);
    };
}

pub struct MemoryMetadata;

impl MemoryMetadata {
    impl_memory_id!(UPGRADES, STABLE_BTREE);
}

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}