pub mod mmgr;
pub mod stable;
pub mod macros;
pub mod common;
pub mod random;
pub mod id_generator;

pub mod statistics {
    /// everything metrics trait. every type that implements [Metrics] must implement metrics with this type
    /// basically stack all the metrics together
    pub trait OpaquePrometheusMetrics {
        fn measure(&self) -> String;
    }

    /// general metrics trait for data structures, M here is actually just a
    /// placeholder for usecase, similar to [core::marker::PhantomData]
    pub trait Metrics<M> {
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

        pub mod canister_statistics {
            use ic_cdk::api::stable::WASM_PAGE_SIZE_IN_BYTES;

            use crate::statistics::Metrics;

            pub const WASM_PAGE_SIZE: u64 = WASM_PAGE_SIZE_IN_BYTES as u64;

            /// canister memory statistics, only works in wasm environment
            pub struct MemoryStatistics;

            impl MemoryStatistics {
                pub fn get_heap_size() -> u64 {
                    fn get_heap_size() -> u64 {
                        #[cfg(target_arch = "wasm32")]
                        {
                            (core::arch::wasm32::memory_size(0) as u64) * WASM_PAGE_SIZE
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            0
                        }
                    }
                }

                pub fn get_stable_size() -> u64 {
                    fn get_stable_size() -> u64 {
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
            }
            struct Stable;
            struct Heap;

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
        }
    }
}
