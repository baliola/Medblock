use std::cell::RefCell;

use ic_stable_structures::{
    memory_manager::{ MemoryId, MemoryManager as IcMemoryManager },
    DefaultMemoryImpl,
};

use crate::common::Get;

pub type CanisterVirtualMemory =
    ic_stable_structures::memory_manager::VirtualMemory<DefaultMemoryImpl>;
pub struct MemoryManager {
    manager: RefCell<IcMemoryManager<DefaultMemoryImpl>>,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::init()
    }
}

impl MemoryManager {
    pub fn get_memory<R, Memory: Get<MemoryId>>(
        &self,
        f: impl FnOnce(CanisterVirtualMemory) -> R
    ) -> R {

        let mem = self.manager.borrow().get(Memory::get());
        

        f(mem)
    }
    pub fn init() -> Self {
        let mgr = IcMemoryManager::init(DefaultMemoryImpl::default());

        Self {
            manager: RefCell::new(mgr),
        }
    }
}

#[macro_export]
macro_rules! memory_manager {
    () => {
        $crate::mmgr::MemoryManager::init()
    };
}
