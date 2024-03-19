use std::{ borrow::Borrow, fmt::Debug, marker::PhantomData, ops::{ DerefMut, RangeBounds } };

use candid::CandidType;
use ic_stable_structures::{ storable::Bound, DefaultMemoryImpl, Storable };
use parity_scale_codec::{ Codec, Decode, Encode };
use serde::{ de::DeserializeOwned, Deserialize, Serialize };

use super::mmgr::MemoryManager;

pub trait MemBoundMarker {
    const BOUND: Bound;
}

pub trait ToStable {
    fn to_stable<Encoding: EncodingMarker>(self) -> Stable<Self, Encoding>
        where Self: Sized + MemBoundMarker
    {
        Stable::new(self)
    }

    fn from_stable<Encoding: EncodingMarker>(stable: Stable<Self, Encoding>) -> Self
        where Self: Sized + MemBoundMarker
    {
        stable.into_inner()
    }
}

impl<T: MemBoundMarker> ToStable for T {}

impl<T: Storable> MemBoundMarker for T {
    const BOUND: Bound = <T as Storable>::BOUND;
}

pub type Memory = ic_stable_structures::memory_manager::VirtualMemory<DefaultMemoryImpl>;

#[derive(Debug)]
pub struct Scale;
#[derive(Debug)]
pub struct Candid;
trait EncodingMarker {}
impl EncodingMarker for Scale {}
impl EncodingMarker for Candid {}

#[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug)]
pub struct Stable<Data, Encoding = Scale>(Data, PhantomData<Encoding>)
    where Data: MemBoundMarker, Encoding: EncodingMarker;

impl<Data, Encoding> From<Data>
    for Stable<Data, Encoding>
    where Data: MemBoundMarker, Encoding: EncodingMarker
{
    fn from(value: Data) -> Self {
        Stable::new(value)
    }
}

impl<Data, Encoding> PartialEq
    for Stable<Data, Encoding>
    where Data: MemBoundMarker + PartialEq, Encoding: EncodingMarker
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other.as_inner())
    }
}

impl<Data, Encoding> Eq
    for Stable<Data, Encoding>
    where Data: MemBoundMarker + Eq, Encoding: EncodingMarker {}

impl<Data, Encoding> Default
    for Stable<Data, Encoding>
    where Data: MemBoundMarker + Default, Encoding: EncodingMarker
{
    fn default() -> Self {
        Stable(Default::default(), PhantomData)
    }
}

impl<Data, Encoding> Stable<Data, Encoding> where Data: MemBoundMarker, Encoding: EncodingMarker {
    pub fn as_inner(&self) -> &Data {
        &self.0
    }
}

impl<Data, Encoding> std::ops::Deref
    for Stable<Data, Encoding>
    where Data: MemBoundMarker, Encoding: EncodingMarker
{
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Data, Encoding> DerefMut
    for Stable<Data, Encoding>
    where Data: MemBoundMarker, Encoding: EncodingMarker
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Data, Encoding> Borrow<Data>
    for Stable<Data, Encoding>
    where Data: MemBoundMarker, Encoding: EncodingMarker
{
    fn borrow(&self) -> &Data {
        &self.0
    }
}

impl<Data, Encoding> RangeBounds<Stable<Data, Encoding>>
    for Stable<Data, Encoding>
    where Encoding: EncodingMarker, Data: MemBoundMarker + RangeBounds<Data>
{
    fn start_bound(&self) -> std::ops::Bound<&Stable<Data, Encoding>> {
        match self.0.start_bound() {
            std::ops::Bound::Included(_) => std::ops::Bound::Included(self),
            std::ops::Bound::Excluded(_) => std::ops::Bound::Excluded(self),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> std::ops::Bound<&Stable<Data, Encoding>> {
        match self.0.end_bound() {
            std::ops::Bound::Included(_) => std::ops::Bound::Included(self),
            std::ops::Bound::Excluded(_) => std::ops::Bound::Excluded(self),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
        }
    }
}

impl<Data, Encoding> Stable<Data, Encoding> where Data: MemBoundMarker, Encoding: EncodingMarker {
    pub fn new(value: Data) -> Self {
        Stable(value, PhantomData)
    }

    pub fn get(&self) -> &Data {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Data {
        &mut self.0
    }

    pub fn into_inner(self) -> Data {
        self.0
    }
}

impl<Data, Encoding> Clone
    for Stable<Data, Encoding>
    where Data: Clone + Encode + MemBoundMarker, Encoding: EncodingMarker
{
    fn clone(&self) -> Self {
        Stable(self.0.clone(), PhantomData)
    }
}

impl<Data, Encoding> PartialOrd
    for Stable<Data, Encoding>
    where Data: PartialOrd + MemBoundMarker, Encoding: EncodingMarker
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<Data, Encoding> Ord
    for Stable<Data, Encoding>
    where Data: Ord + MemBoundMarker, Encoding: EncodingMarker
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<Data> Storable for Stable<Data, Scale> where Data: Codec + Sized + MemBoundMarker {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let buf = <Self as Encode>::encode(self);

        std::borrow::Cow::Owned(buf)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        <Self as Decode>::decode(&mut &*bytes).unwrap()
    }

    const BOUND: Bound = <Data as MemBoundMarker>::BOUND;
}

impl<Data> Storable
    for Stable<Data, Candid>
    where Data: CandidType + DeserializeOwned + Sized + MemBoundMarker
{
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        use candid::Encode;
        let buf = Encode!(&self.0).unwrap();
        std::borrow::Cow::Owned(buf)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        use candid::Decode;
        let buf = Decode!(bytes.as_ref(), Data).unwrap();
        Stable::new(buf)
    }

    const BOUND: Bound = <Data as MemBoundMarker>::BOUND;
}

#[cfg(test)]
mod stable_test {
    use super::*;

    #[test]
    fn test_stable() {
        let stable = Stable::<_>::new(10_u32);
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
        let stable = Stable::<_>::new(vec![1, 2, 3, 4]);
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
        let table_encoded = Stable::<_>::new(str1).encode();

        println!("{:?}", table_encoded.len());
        Stable::<SimilarStruct>::from_bytes(table_encoded.to_bytes());
    }
}

pub struct StableSet<K, V>(ic_stable_structures::BTreeMap<(K, V), (), Memory>)
    where
        K: Storable + Ord + Clone + RangeBounds<K> + Default,
        V: Storable + Ord + Clone + RangeBounds<V> + Default;

impl<K, V> Debug
    for StableSet<K, V>
    where
        K: Storable + Ord + Clone + RangeBounds<K> + Default + Debug,
        V: Storable + Ord + Clone + RangeBounds<V> + Default + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.0.iter().map(|((k, v), _)| ((k.clone(), v.clone()), ())))
            .finish()
    }
}

impl<K, V> StableSet<K, V>
    where
        K: Storable + Ord + Clone + RangeBounds<K> + Default,
        V: Storable + Ord + Clone + RangeBounds<V> + Default
{
    pub fn inner(&self) -> &ic_stable_structures::BTreeMap<(K, V), (), Memory> {
        &self.0
    }

    pub fn len(&self) -> u64 {
        self.0.len()
    }

    pub fn new(memory_manager: &MemoryManager) -> StableSet<K, V> {
        let tree = memory_manager.get_memory(ic_stable_structures::BTreeMap::new);
        Self(tree)
    }

    /// This function checks if a key exists in the set. It does so by creating a range iterator
    /// from the given key to the end of the set. It then checks if the maximum value in the range
    /// iterator is not None. If it is not None, it returns true. Otherwise, it returns false.
    pub fn range_key_exists(&self, partial_key: &K) -> bool {
        self.0
            .range((partial_key.clone(), V::default())..)
            .max()
            .is_some()
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

        for (k, _v) in range {
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

    // This function retrieves a page of unique values associated with a given key from a sorted set.
    // The set is sorted by key and value, and each key can be associated with multiple values.
    //
    // The function takes a key, a page number, and a limit as arguments.
    // The page number and limit are used to calculate the start and end indices of the page.
    //
    // The function starts by creating a range iterator from the given key to the end of the set.
    // It then iterates over the set, keeping track of the last value it has seen and the current index.
    //
    // For each entry in the set, it checks if the key of the entry is the same as the given key and if the index is within the page range.
    // If the key is different or the index is outside the page range, it breaks the loop.
    //
    // If the value of the entry is different from the last value it has seen and the index is within the page range, it adds the value to the result and increments the index.
    // If the value is different but the index is outside the page range, it just increments the index.
    //
    // After the loop, if the result is empty, it returns None. Otherwise, it returns the result wrapped in Some.
    //
    // This way, the function returns a page of unique values associated with the given key, or None if there are no such values.
    pub fn get_set_associated_by_key_paged(
        &self,
        key: &K,
        page: u64,
        limit: u64
    ) -> Option<Vec<V>> {
        let start = page * limit;
        let end = start + limit;

        let mut last_id = V::default();
        let mut index = 0;

        let iter = self.0.range((key.clone(), V::default())..);

        let mut result = vec![];

        // Iterate over the range iterator
        for ((k, v), _) in iter {
            // If the key of the current entry is not the same as the provided key,
            // or if the current index has reached or exceeded the end of the page,
            // break the loop. This is to ensure that we only process entries with the
            // provided key and within the specified page.
            if k != *key || index >= end {
                break;
            }

            // If the value of the current entry is not the same as the last seen value
            // and the current index has reached or exceeded the start of the page,
            // add the value to the result, update the last seen value, and increment the index.
            // This is to ensure that we only add unique values to the result and only after
            // we have reached the start of the page.
            if v != last_id && index >= start {
                result.push(v.clone());
                last_id = v.clone();
                index += 1;
            } else if
                // If the value of the current entry is not the same as the last seen value,
                // but the current index has not yet reached the start of the page,
                // update the last seen value and increment the index.
                // This is to skip over any unique values that fall before the start of the page.
                v != last_id
            {
                last_id = v.clone();
                index += 1;
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

    pub fn contains_key(&self, key: K, value: V) -> bool {
        self.0.contains_key(&(key, value))
    }
}

#[cfg(test)]
mod set_test {
    use crate::{ memory_manager, native_bound };

    use super::*;

    native_bound!(u8, u32);

    #[test]
    fn test_stable_set() {
        let memor_manager = memory_manager!();

        let mut set = StableSet::<Stable<Nativeu8>, Stable<Nativeu8>>::new(&memor_manager);

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

    #[test]
    fn test_paged_query() {
        let memor_manager = memory_manager!();

        let mut set = StableSet::<Stable<Nativeu8>, Stable<Nativeu8>>::new(&memor_manager);

        let value = [Nativeu8(10), Nativeu8(20), Nativeu8(30), Nativeu8(40)].to_vec();
        let key = Nativeu8(10);

        for v in value.iter() {
            set.insert(key.clone().to_stable(), v.clone().to_stable());
        }

        let result = set.get_set_associated_by_key_paged(&Nativeu8(10).to_stable(), 0, 2).unwrap();
        let initial_value = value.into_iter().map(ToStable::to_stable).take(2).collect::<Vec<_>>();
        let expected_value = result;

        assert_eq!(initial_value, expected_value);
    }

    #[test]
    fn test_paged_query_with_wrong_keys() {
        let memor_manager = memory_manager!();

        let mut set = StableSet::<Stable<Nativeu8>, Stable<Nativeu8>>::new(&memor_manager);

        let value = [Nativeu8(10), Nativeu8(20), Nativeu8(30), Nativeu8(40)].to_vec();
        let key = Nativeu8(10);

        for v in value.iter() {
            set.insert(key.clone().to_stable(), v.clone().to_stable());
        }

        let result = set.get_set_associated_by_key_paged(&Nativeu8(20).to_stable(), 0, 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_paged_query_with_mixed_keys() {
        let memor_manager = memory_manager!();

        let mut set = StableSet::<Stable<Nativeu8>, Stable<Nativeu8>>::new(&memor_manager);

        let value = [Nativeu8(10), Nativeu8(20), Nativeu8(30), Nativeu8(40)].to_vec();
        let key = Nativeu8(10);

        for v in value.iter() {
            set.insert(key.clone().to_stable(), v.clone().to_stable());
        }

        let wrong_key = Nativeu8(20);
        let wrong_value = [Nativeu8(56), Nativeu8(78), Nativeu8(90), Nativeu8(100)].to_vec();

        for v in wrong_value.iter() {
            set.insert(wrong_key.clone().to_stable(), v.clone().to_stable());
        }

        let result = set.get_set_associated_by_key_paged(&Nativeu8(10).to_stable(), 0, 2).unwrap();
        let initial_value = value.into_iter().map(ToStable::to_stable).take(2).collect::<Vec<_>>();
        let expected_value = result;

        assert_eq!(initial_value, expected_value);
    }
}
