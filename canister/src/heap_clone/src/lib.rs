use std::collections::{ BTreeMap, HashMap };

use ic_stable_memory::{
    StableType,
    AsDynSizeBytes,
    SBox,
    AsFixedSizeBytes,
    collections::{ SVec, SLog, SHashMap, SHashSet, SBTreeMap, SBTreeSet },
};

pub trait HeapClone {
    type Target;

    fn clone_heap(&self) -> Self::Target;
}

macro_rules! impl_heap_clone {
    ($($ty:ty),*) => {
        
        $(
            impl HeapClone for $ty {
                type Target = $ty;

                fn clone_heap(&self) -> Self::Target {
                    self.clone()
                }
            }
        )*
    };
}

impl_heap_clone!(
    (),
    bool,
    char,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
    String
    // std::collections::HashMap<K, V>,
);

impl<K: Clone, V: Clone> HeapClone for BTreeMap<K, V> {
    type Target = BTreeMap<K, V>;

    fn clone_heap(&self) -> Self::Target {
        self.clone()
    }
}

impl<K: Clone, V: Clone> HeapClone for HashMap<K, V> {
    type Target = HashMap<K, V>;

    fn clone_heap(&self) -> Self::Target {
        self.clone()
    }
}

impl<T: Clone> HeapClone for Box<T> {
    type Target = Box<T>;

    fn clone_heap(&self) -> Self::Target {
        self.clone()
    }
}

impl<T: Clone> HeapClone for Vec<T> {
    type Target = Vec<T>;

    fn clone_heap(&self) -> Self::Target {
        self.clone()
    }
}

impl<T: Clone> HeapClone for std::collections::BTreeSet<T> {
    type Target = std::collections::BTreeSet<T>;

    fn clone_heap(&self) -> Self::Target {
        self.clone()
    }
}

impl<T: Clone> HeapClone for std::collections::HashSet<T> {
    type Target = std::collections::HashSet<T>;

    fn clone_heap(&self) -> Self::Target {
        self.clone()
    }
}

impl<T: Clone> HeapClone for Option<T> {
    type Target = Option<T>;

    fn clone_heap(&self) -> Self::Target {
        self.clone()
    }
}

impl<T: Clone, E: Clone> HeapClone for Result<T, E> {
    type Target = Result<T, E>;

    fn clone_heap(&self) -> Self::Target {
        self.clone()
    }
}

impl<T: AsDynSizeBytes + StableType> HeapClone for SBox<T> {
    type Target = Box<T>;

    fn clone_heap(&self) -> Self::Target {
        let buf = self.as_dyn_size_bytes();
        let data = T::from_dyn_size_bytes(&buf);
        Box::new(data)
    }
}

impl<T: AsFixedSizeBytes + StableType + Clone> HeapClone for SVec<T> {
    type Target = Vec<T>;

    fn clone_heap(&self) -> Self::Target {
        self.iter()
            .map(|v| v.clone())
            .collect()
    }
}

impl<T: AsFixedSizeBytes + StableType + Clone> HeapClone for SLog<T> {
    type Target = Vec<T>;

    fn clone_heap(&self) -> Self::Target {
        self.rev_iter()
            .map(|v| v.clone())
            .collect()
    }
}

impl<K, V> HeapClone
    for SHashMap<K, V>
    where
        K: StableType + AsFixedSizeBytes + std::hash::Hash + Eq + Clone,
        V: StableType + AsFixedSizeBytes + Clone
{
    type Target = HashMap<K, V>;

    fn clone_heap(&self) -> Self::Target where Self: Sized {
        self.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<HashMap<K, V>>()
            .into()
    }
}

impl<K> HeapClone
    for SHashSet<K>
    where K: StableType + AsFixedSizeBytes + std::hash::Hash + Eq + Clone
{
    type Target = std::collections::HashSet<K>;

    fn clone_heap(&self) -> Self::Target where Self: Sized {
        self.iter()
            .map(|v| v.clone())
            .collect::<std::collections::HashSet<K>>()
    }
}

impl<K, V> HeapClone
    for SBTreeMap<K, V>
    where
        K: StableType + AsFixedSizeBytes + std::cmp::Ord + Clone,
        V: StableType + AsFixedSizeBytes + Clone
{
    type Target = BTreeMap<K, V>;

    fn clone_heap(&self) -> Self::Target where Self: Sized {
        self.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<BTreeMap<K, V>>()
            .into()
    }
}

impl<K> HeapClone for SBTreeSet<K> where K: StableType + AsFixedSizeBytes + std::cmp::Ord + Clone {
    type Target = std::collections::BTreeSet<K>;

    fn clone_heap(&self) -> Self::Target where Self: Sized {
        self.iter()
            .map(|v| v.clone())
            .collect::<std::collections::BTreeSet<K>>()
    }
}
