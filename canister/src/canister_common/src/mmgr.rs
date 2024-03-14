use std::cell::RefCell;

use ic_stable_structures::{
    memory_manager::{ MemoryId, MemoryManager as IcMemoryManager },
    DefaultMemoryImpl,
};

pub type CanisterVirtualMemory =
    ic_stable_structures::memory_manager::VirtualMemory<DefaultMemoryImpl>;
pub struct MemoryManager {
    manager: RefCell<IcMemoryManager<DefaultMemoryImpl>>,
    index: RefCell<u8>,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryManager {
    pub fn get_memory<T>(&self, f: impl FnOnce(CanisterVirtualMemory) -> T) -> T {

        let mut index = self.index.borrow_mut();

        let mem = self.manager.borrow().get(MemoryId::new(*index));
        let result = f(mem);

        *index += 1;

        result
    }
    pub fn new() -> Self {
        let mgr = IcMemoryManager::init(DefaultMemoryImpl::default());

        Self {
            manager: RefCell::new(mgr),
            index: RefCell::new(0),
        }
    }
}


#[macro_export]
macro_rules! memory_manager {
    () => {
        $crate::mmgr::MemoryManager::new()
    };
}
