use std::{ borrow::Borrow, ops::RangeBounds };

use ic_stable_structures::{ storable::Bound, DefaultMemoryImpl, Storable };
use parity_scale_codec::{ Codec, Decode, Encode };

use super::MemoryManager;

pub trait MemBoundMarker {
    const BOUND: Bound;
}

pub trait ToStable {
    fn to_stable(self) -> Stable<Self> where Self: Sized + MemBoundMarker {
        Stable::new(self)
    }

    fn from_stable(stable: Stable<Self>) -> Self where Self: Sized + MemBoundMarker {
        stable.into_inner()
    }
}

impl<T: MemBoundMarker> ToStable for T {}

impl<T: Storable> MemBoundMarker for T {
    const BOUND: Bound = <T as Storable>::BOUND;
}

pub type Memory = ic_stable_structures::memory_manager::VirtualMemory<DefaultMemoryImpl>;

#[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug)]
pub struct Stable<T>(T) where T: MemBoundMarker;

impl<T> PartialEq for Stable<T> where T: MemBoundMarker + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other.as_inner())
    }
}

impl<T> Eq for Stable<T> where T: MemBoundMarker + Eq {}

impl<T> Default for Stable<T> where T: MemBoundMarker + Default {
    fn default() -> Self {
        Stable(Default::default())
    }
}

impl<T> Stable<T> where T: MemBoundMarker {
    pub fn as_inner(&self) -> &T {
        &self.0
    }
}

impl<T> std::ops::Deref for Stable<T> where T: MemBoundMarker {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Borrow<T> for Stable<T> where T: MemBoundMarker {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T: MemBoundMarker + RangeBounds<T>> RangeBounds<Stable<T>> for Stable<T> {
    fn start_bound(&self) -> std::ops::Bound<&Stable<T>> {
        match self.0.start_bound() {
            std::ops::Bound::Included(_) => std::ops::Bound::Included(self),
            std::ops::Bound::Excluded(_) => std::ops::Bound::Excluded(self),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> std::ops::Bound<&Stable<T>> {
        match self.0.end_bound() {
            std::ops::Bound::Included(_) => std::ops::Bound::Included(self),
            std::ops::Bound::Excluded(_) => std::ops::Bound::Excluded(self),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        }
    }
}

impl<T: MemBoundMarker> From<T> for Stable<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Stable<T> where T: MemBoundMarker {
    pub fn new(value: T) -> Self {
        Stable(value)
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Clone for Stable<T> where T: Clone + Encode + MemBoundMarker {
    fn clone(&self) -> Self {
        Stable(self.0.clone())
    }
}

impl<T> PartialOrd for Stable<T> where T: PartialOrd + MemBoundMarker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for Stable<T> where T: Ord + MemBoundMarker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Storable for Stable<T> where T: Codec + Sized + MemBoundMarker {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let buf = <Self as Encode>::encode(self);

        std::borrow::Cow::Owned(buf)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        <Self as Decode>::decode(&mut &*bytes).unwrap()
    }

    const BOUND: Bound = <T as MemBoundMarker>::BOUND;
}

#[cfg(test)]
mod stable_test {
    use super::*;

    #[test]
    fn test_stable() {
        let stable = Stable::new(10_u32);
        let bytes = stable.to_bytes();
        let stable2 = Stable::from_bytes(bytes);
        assert_eq!(stable, stable2);
    }

    #[test]
    fn test_struct() {
        #[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug, PartialEq, Eq)]
        struct TestStruct {
            a: u32,
            b: u32,
        }

        impl MemBoundMarker for TestStruct {
            const BOUND: Bound = Bound::Unbounded;
        }

        let stable = Stable::new(TestStruct { a: 10, b: 20 });

        let bytes = stable.to_bytes();
        let stable2 = Stable::from_bytes(bytes);
        assert_eq!(stable, stable2);
    }

    #[test]
    fn test_unbounded_vec() {
        let stable = Stable::new(vec![1, 2, 3, 4]);
        let bytes = stable.to_bytes();
        let stable2 = Stable::from_bytes(bytes);
        assert_eq!(stable, stable2);
    }

    #[test]
    #[should_panic]
    fn test_similar_struct_codec() {
        #[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug, PartialEq, Eq)]
        struct TestStruct {
            a: u32,
            b: u32,
        }

        #[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug, PartialEq, Eq)]
        struct SimilarStruct {
            a: u32,
            b: u32,
            optional: Option<u32>,
        }

        impl MemBoundMarker for TestStruct {
            const BOUND: Bound = Bound::Unbounded;
        }

        impl MemBoundMarker for SimilarStruct {
            const BOUND: Bound = Bound::Unbounded;
        }

        let str1 = TestStruct { a: 10, b: 20 };
        let table_encoded = Stable::new(str1).encode();

        Stable::<SimilarStruct>::from_bytes(table_encoded.to_bytes());
    }
}

pub struct StableSet<K, V>(ic_stable_structures::BTreeMap<(K, V), (), Memory>)
    where
        K: Storable + Ord + Clone + RangeBounds<K> + Default,
        V: Storable + Ord + Clone + RangeBounds<V> + Default;

impl<K, V> StableSet<K, V>
    where
        K: Storable + Ord + Clone + RangeBounds<K> + Default,
        V: Storable + Ord + Clone + RangeBounds<V> + Default
{
    pub fn inner(&self) -> &ic_stable_structures::BTreeMap<(K, V), (), Memory> {
        &self.0
    }

    pub fn new(memory_manager: MemoryManager) -> StableSet<K, V> {
        let tree = memory_manager.get_memory(ic_stable_structures::BTreeMap::new);
        Self(tree)
    }

    pub fn inner_mut(&mut self) -> &mut ic_stable_structures::BTreeMap<(K, V), (), Memory> {
        &mut self.0
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.0.insert((k, v), ());
    }

    pub fn get_set_associated_by_key(&self, key: &K) -> Option<Vec<V>> {
        let mut result = vec![];

        let range = self.0.range((key.clone(), V::default())..);

        for (k, v) in range {
            if k.0 == *key {
                result.push(k.1.clone());
            }

            // short circuit if we have moved to the next key
            if k.0 != *key {
                break;
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn total_associated_of_key(&self, key: &K) -> usize {
        let range = self.0.range((key.clone(), V::default())..);

        let mut count = 0;

        for (k, _) in range {
            if k.0 == *key {
                count += 1;
            }

            // short circuit if we have moved to the next key
            if k.0 != *key {
                break;
            }
        }

        count
    }
}

#[cfg(test)]
mod set_test {
    use crate::{fake_memory_manager, impl_max_size, impl_mem_bound};
    use paste::paste;
    use super::*;

    macro_rules! native_bound {
        ($($ident:ident),*) => {
            $(
                paste!{

                #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, Default)]
                pub struct [<Native $ident>]($ident);

               impl_max_size!(for [<Native $ident>]: $ident);
               impl_mem_bound!(for [<Native $ident>]: bounded; fixed_size: true);

                impl RangeBounds<[<Native $ident>]> for [<Native $ident>] {
                    fn start_bound(&self) -> std::ops::Bound<&[<Native $ident>]> {
                        std::ops::Bound::Included(self)
                    }

                    fn end_bound(&self) -> std::ops::Bound<&[<Native $ident>]> {
                        std::ops::Bound::Excluded(self)
                    }
                }

                impl From<$ident> for [<Native $ident>] {
                    fn from(value: $ident) -> Self {
                        [<Native $ident>](value)
                    }
                }

                impl Into<$ident> for [<Native $ident>] {
                    fn into(self) -> $ident {
                        self.0
                    }
                }
                }

            )*
        };
    }

    native_bound!(u8, u32);
    

    #[test]
    fn test_stable_set() {
        let memor_manager = fake_memory_manager!();

        let mut set = StableSet::<Stable<Nativeu8>, Stable<Nativeu8>>::new(memor_manager);

        let value = [Nativeu8(10), Nativeu8(20), Nativeu8(30), Nativeu8(40)].to_vec();
        let key = Nativeu8(10);
        
        for v in value.iter() {
            set.insert(key.clone().to_stable(), v.clone().to_stable());
        }
        
        let result = set.get_set_associated_by_key(&Nativeu8(10).to_stable()).unwrap();
        let initial_value = value.into_iter().map(ToStable::to_stable).collect::<Vec<_>>();
        let expected_value = result;

        assert_eq!(initial_value, expected_value);

        let total = set.total_associated_of_key(&Nativeu8(10).to_stable());
        assert_eq!(total, expected_value.len());
    }
}
