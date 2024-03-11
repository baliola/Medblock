pub mod mmgr;
pub mod stable;
mod macros;
pub mod common;
pub mod random;

pub mod alloc {
    pub use tikv_jemallocator::Jemalloc as Allocator;
    pub use tikv_jemalloc_ctl as ctl;

    #[macro_export]
    macro_rules! impl_allocation_statistics {
        () => {
            #[global_allocator]
            static GLOBAL: $crate::alloc::Allocator = $crate::alloc::Allocator;
        };
    }



    pub trait Metrics {
        fn metrics_name() -> &'static str;

        fn metrics_measurements() -> &'static str;

        fn prometheus_id() -> String {
            format!("{}_{}", Self::metrics_name(), Self::metrics_measurements())
        }

        fn update_measurements(&self);

        fn cached_allocated_size(&self) -> usize;
    }
}
