use ic_stable_memory::{ AsDynSizeBytes, SBox, StableType };

use crate::deref;

use super::{ HeapCloneMut, HeapMarker };

#[derive(Clone, Debug, Default)]
pub struct HBox<T>(Box<T>);

impl<T> std::ops::Deref for HBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl<T> From<T> for HBox<T> {
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

impl<T> From<Box<T>> for HBox<T> {
    fn from(value: Box<T>) -> Self {
        Self(value)
    }
}

impl<T> From<SBox<T>> for HBox<T> where T: AsDynSizeBytes + StableType {
    fn from(value: SBox<T>) -> Self {
        Self(value.into_inner().into())
    }
}

impl<T> HeapCloneMut for HBox<T> where T: HeapMarker + Clone {
    type Target = Self;

    fn clone_heap_mut(&mut self) -> Self::Target where Self: Sized {
        self.clone()
    }
}

impl<T> HeapCloneMut for SBox<T> where T: AsDynSizeBytes + StableType + Clone {
    type Target = HBox<T>;

    fn clone_heap_mut(&mut self) -> Self::Target where Self: Sized {
        // TODO : find if placing unwrap here is safe
        self.with(|x| HBox::from(x.clone())).unwrap()
    }
}

impl<T> HeapMarker for HBox<T> {}
