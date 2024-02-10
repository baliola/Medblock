use std::marker::PhantomData;

use ic_stable_structures::{ storable::Bound, Storable };
use parity_scale_codec::{ Codec, Decode, Encode };
use crate::{ deref, impl_unbounded };

pub trait MemBoundMarker {
    const BOUND: Bound;
}

impl<T: Storable> MemBoundMarker for T {
    const BOUND: Bound = <T as Storable>::BOUND;
}

#[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug)]
pub struct Stable<T>(T) where T: MemBoundMarker;

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


}
