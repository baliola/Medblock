pub trait HeapClone {
    type Target;

    fn clone(&self) -> Self::Target;
}

/// for boxed types
pub trait HeapCloneMut {
    type Target;

    fn clone_heap_mut(&mut self) -> Self::Target;
}
