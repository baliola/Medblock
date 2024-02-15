use ic_stable_structures::{ storable::Bound, DefaultMemoryImpl, Storable };
use parity_scale_codec::{ Codec, Decode, Encode };

use crate::deref;

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

impl<T> std::ops::Deref for Stable<T> where T: MemBoundMarker {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
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

impl<T> PartialEq for Stable<T> where T: PartialEq + MemBoundMarker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Stable<T> where T: Eq + MemBoundMarker {}

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
mod test {
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
