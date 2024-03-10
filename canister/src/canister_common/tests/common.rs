use std::ops::RangeBounds;

use canister_common::memory_manager;
use ic_stable_structures::{memory_manager, Storable};

canister_common::native_bound!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

pub fn stable_set<K, V>() -> canister_common::stable::StableSet<K, V>
    where
        K: Storable + Ord + Clone + RangeBounds<K> + Default,
        V: Storable + Ord + Clone + RangeBounds<V> + Default
{
    canister_common::stable::StableSet::new(memory_manager!())
}
