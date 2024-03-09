pub mod mmgr;
pub mod stable;
mod macros;
pub mod common;

pub mod alloc {
    pub use tikv_jemallocator::Jemalloc as Allocator;
    pub use tikv_jemalloc_ctl as ctl;

    #[macro_export]
    macro_rules! setup_allocator {
        () => {
            #[global_allocator]
            static GLOBAL: $crate::alloc::Allocator = $crate::alloc::Allocator;
        };
    }
}
