use crate::metrics::BEACON_STATE_MEMORY_SIZE_CALCULATION_TIME;
use milhouse::mem::{MemorySize, MemoryTracker};
use std::time::Instant;
use types::{BeaconState, EthSpec};

/// BeaconState Wrapper for memory tracking.
pub struct BeaconStateWrapper<'a, E: EthSpec>(pub &'a BeaconState<E>);

impl<'a, E: EthSpec> MemorySize for BeaconStateWrapper<'a, E> {
    fn self_pointer(&self) -> usize {
        self.0 as *const _ as usize
    }

    fn subtrees(&self) -> Vec<&dyn MemorySize> {
        vec![]
    }

    fn intrinsic_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

/// Extension trait for approximate memory consumption of a `BeaconState`.
pub trait BeaconStateMemorySize {
    fn memory_size(&self) -> usize;
}

impl<E: EthSpec> BeaconStateMemorySize for BeaconState<E> {
    fn memory_size(&self) -> usize {
        let wrapper = BeaconStateWrapper(self);
        // Timer for MemorySize
        let timer = Instant::now();
        // Use MemoryTracker on the wrapper
        let mut tracker = MemoryTracker::default();
        let stats = tracker.track_item(&wrapper);

        let elapsed_time = timer.elapsed();
        metrics::observe(
            &BEACON_STATE_MEMORY_SIZE_CALCULATION_TIME,
            elapsed_time.as_secs_f64(),
        );
        stats.differential_size
    }
}
