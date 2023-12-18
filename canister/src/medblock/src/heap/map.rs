use std::{ collections::{ HashMap, BTreeMap }, hash::Hash };

use ic_stable_memory::{ collections::SHashMap, AsFixedSizeBytes, StableType };

use super::{ HeapClone, HeapMarker };

#[derive(Debug, Default)]
pub struct HHashMap<K, V>(HashMap<K, V>);

impl<K, V> std::ops::Deref for HHashMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> std::ops::DerefMut for HHashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<K, V> From<HashMap<K, V>> for HHashMap<K, V> {
    fn from(value: HashMap<K, V>) -> Self {
        Self(value)
    }
}

impl<K, V> From<SHashMap<K, V>>
    for HHashMap<K, V>
    where
        K: StableType + AsFixedSizeBytes + Hash + Eq + Clone,
        V: StableType + AsFixedSizeBytes + Clone
{
    fn from(value: SHashMap<K, V>) -> Self {
        // this invloves copying the data first and then dropping the stable memory
        Self(
            value
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<K, V>>()
        )
    }
}

impl<K, V> HeapClone
    for SHashMap<K, V>
    where
        K: StableType + AsFixedSizeBytes + Hash + Eq + Clone,
        V: StableType + AsFixedSizeBytes + Clone
{
    type Target = HHashMap<K, V>;

    fn clone_heap(&self) -> Self::Target where Self: Sized {
        self.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<HashMap<K, V>>()
            .into()
    }
}
impl<K, V> HeapMarker for HHashMap<K, V> {}

// pub struct HBTreeMap(BTreeMap<>);
