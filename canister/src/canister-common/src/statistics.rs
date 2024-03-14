/// metrics macro, used to define metrics for data structures
#[macro_export]
macro_rules! metrics {
    ($($ident:ident),*) => {
        $(struct $ident;

            impl $crate::statistics::traits::MetricsMarker for $ident {}
        )*
    };
}

pub mod traits {
    pub trait MetricsMarker {}
    /// everything metrics trait. every type that implements [Metrics] must implement metrics with this type
    /// as this type basically stack all the metrics together
    pub trait OpaqueMetrics {
        /// stack all metrics implemented using [Metrics] together. meant to be used in prometheus and http response
        fn measure(&self) -> String;

        /// update the measurements of metrics, should call all internal [Metrics] implementation
        fn update(&self);
    }

    /// general metrics trait for data structures, M here is actually just a
    /// placeholder for usecase, similar to [core::marker::PhantomData]
    pub trait Metrics<M: MetricsMarker> {
        /// the metrics name, e.g. map_size. DONT put the measurements here
        ///
        /// wrong : "map_size_bytes"
        ///
        /// right : "map_size"
        fn metrics_name() -> &'static str;

        /// the measurements of the metrics, e.g. "bytes", "len"
        fn metrics_measurements() -> &'static str;

        fn prometheus_id() -> String {
            format!("{}_{}", Self::metrics_name(), Self::metrics_measurements())
        }

        /// update the measurements of the metrics, useful for metrics that must be owned by certain data structrues
        fn update_measurements(&self);

        /// get the measurements of the metrics, e.g. "100" for 100 bytes
        fn get_measurements(&self) -> String;

        /// prometheus styled metrics, e.g. "WASM_PAGE_SIZE 100", "value_size 200" etc.
        /// meant to be stacked with other type that implement metrics
        fn measure(&self) -> String {
            format!("{} {}", Self::prometheus_id(), self.get_measurements())
        }
    }

    pub mod canister_statistics {
        use ic_cdk::api::stable::WASM_PAGE_SIZE_IN_BYTES;

        use super::{ Metrics, OpaqueMetrics };

        pub const WASM_PAGE_SIZE: u64 = WASM_PAGE_SIZE_IN_BYTES as u64;

        /// canister memory statistics, only works in wasm environment
        pub struct MemoryStatistics;
        metrics!(Stable, Heap);

        impl MemoryStatistics {
            pub fn get_heap_size() -> u64 {
                #[cfg(target_arch = "wasm32")]
                {
                    (core::arch::wasm32::memory_size(0) as u64) * WASM_PAGE_SIZE
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    0
                }
            }

            pub fn get_stable_size() -> u64 {
                #[cfg(target_arch = "wasm32")]
                {
                    ic_cdk::api::stable::stable_size() * WASM_PAGE_SIZE
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    0
                }
            }
        }

        impl Metrics<Stable> for MemoryStatistics {
            fn metrics_name() -> &'static str {
                "stable_size"
            }

            fn metrics_measurements() -> &'static str {
                "bytes"
            }

            fn update_measurements(&self) {
                // no-op
            }

            fn get_measurements(&self) -> String {
                MemoryStatistics::get_stable_size().to_string()
            }
        }

        impl Metrics<Heap> for MemoryStatistics {
            fn metrics_name() -> &'static str {
                "heap_size"
            }

            fn metrics_measurements() -> &'static str {
                "bytes"
            }

            fn update_measurements(&self) {
                // no-op
            }

            fn get_measurements(&self) -> String {
                MemoryStatistics::get_heap_size().to_string()
            }
        }

        impl OpaqueMetrics for MemoryStatistics {
            fn measure(&self) -> String {
                [Metrics::<Stable>::measure(self), Metrics::<Heap>::measure(self)].join("\n")
            }

            fn update(&self) {
                // no-op
            }
        }

        pub struct BlockchainMetrics;
        metrics!(CycleBalance, CycleBurnt);

        // todo : implement freezing cycles threshold

        impl Metrics<CycleBalance> for BlockchainMetrics {
            fn metrics_name() -> &'static str {
                "balance"
            }

            fn metrics_measurements() -> &'static str {
                "cycles"
            }

            fn update_measurements(&self) {
                // no-op
            }

            fn get_measurements(&self) -> String {
                #[cfg(target_arch = "wasm32")]
                {
                    ic_cdk::api::canister_balance128().to_string();
                }

                #[cfg(not(target_arch = "wasm32"))]
                ({ 0 }).to_string()
            }
        }

        impl Metrics<CycleBurnt> for BlockchainMetrics {
            fn metrics_name() -> &'static str {
                "burnt"
            }

            fn metrics_measurements() -> &'static str {
                "cycles"
            }

            fn update_measurements(&self) {
                // no-op
            }

            fn get_measurements(&self) -> String {
                // TODO : will implement later
                String::from("0")
            }
        }

        impl OpaqueMetrics for BlockchainMetrics {
            fn measure(&self) -> String {
                [Metrics::<CycleBalance>::measure(self), Metrics::<CycleBurnt>::measure(self)].join(
                    "\n"
                )
            }

            fn update(&self) {
                // no-op
            }
        }

        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn test_memory_statistics() {
                assert_eq!(MemoryStatistics::get_heap_size(), 0);
                assert_eq!(MemoryStatistics::get_stable_size(), 0);
            }

            // just to debug print
            #[test]
            #[should_panic]
            fn test_blockchain_metrics() {
                panic!("{}", <MemoryStatistics as OpaqueMetrics>::measure(&MemoryStatistics));
            }
        }
    }
}
