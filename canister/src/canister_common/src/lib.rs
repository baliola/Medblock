pub mod mmgr;
pub mod stable;
pub mod macros;
pub mod common;
pub mod random;
pub mod id_generator;

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

    /// everything metrics trait. every type that implements [Metrics] must implement metrics with this type
    pub trait OpaqueMetrics {
        fn measure(&self) -> String;
    }

    /// general metrics trait for data structures, M here is actually just a
    /// placeholder for usecase, similar to [core::marker::PhantomData]
    pub trait Metrics<M> {
        fn metrics_name() -> &'static str;

        fn metrics_measurements() -> &'static str;

        fn prometheus_id() -> String {
            format!("{}_{}", Self::metrics_name(), Self::metrics_measurements())
        }

        fn update_measurements(&self);

        fn get_measurements(&self) -> String;

        /// prometheus styled metrics, e.g. "key_size 100", "value_size 200" etc.
        /// meant to be stacked with other type that implement metrics
        fn measure(&self) -> String {
            format!("{} {}", Self::prometheus_id(), self.get_measurements())
        }
    }
}
