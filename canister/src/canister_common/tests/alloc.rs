mod common;
use std::{ ops::Add, thread, time::Duration };

use canister_common::{ impl_allocation_statistics, stable::{ Stable, ToStable } };
use common::Nativeu8;
use tikv_jemalloc_ctl::{ epoch, stats };

impl_allocation_statistics!();

#[test]
fn test_stable_alloc_stats() {
    let mut index = 0;
    let mut set = common::stable_set::<Stable<Nativeu8>, Stable<Nativeu8>>();
    set.insert(Nativeu8(index.clone()).to_stable(), Nativeu8(index.add(1)).to_stable());

    index = index.add(1);

    loop {
        // many statistics are cached and only updated when the epoch is advanced.
        epoch::advance().unwrap();
        set.insert(Nativeu8(index.clone()).to_stable(), Nativeu8(index.add(1)).to_stable());
        index = index.add(1);

        let allocated = stats::allocated::read().unwrap();
        let allocated_mb = (allocated as f64) / 1024.0 / 1024.0;
        let resident = stats::resident::read().unwrap();
        println!("{} mb allocated/{} bytes resident", allocated_mb, resident);
        println!("data : {:?}", set);
        // thread::sleep(Duration::from_secs(2));
    }
}
