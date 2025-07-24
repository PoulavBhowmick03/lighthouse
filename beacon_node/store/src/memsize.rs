use milhouse::MemorySize;
use types::{BeaconState, EthSpec};

/// Extension trait to obtain the memory usage of a `BeaconState`.
pub trait BeaconStateMemorySize {
    /// Approximate memory consumption in bytes.
    fn memory_size(&self) -> usize;
}

impl<E: EthSpec> BeaconStateMemorySize for BeaconState<E> {
    fn memory_size(&self) -> usize {
        // Use SSZ length as a coarse approximation.
        self.as_ssz_bytes().len()
    }
}
