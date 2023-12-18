use ic_stable_memory::StableType;

mod boxed;

pub struct Heap<T>(T)
where
    T: HeapMarker;

impl<T> HeapMarker for Heap<T> where T: HeapMarker {}

pub trait HeapMarker {}

pub trait HeapCloneMut {
    type Target: HeapMarker;

    fn clone_heap_mut(&mut self) -> Self::Target
    where
        Self: Sized;
}

pub trait HeapClone {
    type Target: HeapMarker;

    fn clone_heap(&self) -> Self::Target
    where
        Self: Sized;
}
