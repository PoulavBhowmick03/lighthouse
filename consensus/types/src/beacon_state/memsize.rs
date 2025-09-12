use crate::{
    BeaconState, BeaconStateAltair, BeaconStateBase, BeaconStateBellatrix, BeaconStateCapella,
    BeaconStateDeneb, BeaconStateElectra, BeaconStateFulu, BeaconStateGloas, Eth1Data, EthSpec,
    Hash256, List, ParticipationFlags, PendingAttestation, PendingConsolidation, PendingDeposit,
    PendingPartialWithdrawal, Validator, Vector, historical_summary::HistoricalSummary,
};
use milhouse::mem::MemorySize;

use super::{
    map_beacon_state_altair_tree_list_fields_immutable,
    map_beacon_state_base_tree_list_fields_immutable,
    map_beacon_state_bellatrix_tree_list_fields_immutable,
    map_beacon_state_capella_tree_list_fields_immutable,
    map_beacon_state_deneb_tree_list_fields_immutable,
    map_beacon_state_electra_tree_list_fields_immutable,
    map_beacon_state_fulu_tree_list_fields_immutable,
    map_beacon_state_gloas_tree_list_fields_immutable,
};

impl<E: EthSpec> MemorySize for BeaconState<E> {
    fn self_pointer(&self) -> usize {
        self as *const _ as usize
    }

    // Traverse into selected nested containers whose element types we can account for.
    // For containers of foreign element types (e.g., Hash256/B256) we avoid traversal and
    // account for them in intrinsic_size via len * size_of::<T>().
    fn subtrees<'a>(&'a self) -> Vec<&'a (dyn MemorySize + 'a)> {
        let mut subtrees: Vec<&'a (dyn MemorySize + 'a)> = vec![];
        match self {
            Self::Base(self_inner) => {
                map_beacon_state_base_tree_list_fields_immutable!(
                    &'a _,
                    self_inner,
                    |_, self_field| {
                        subtrees.push(self_field);
                    }
                );
            }
            Self::Altair(self_inner) => {
                map_beacon_state_altair_tree_list_fields_immutable!(
                    &'a _,
                    self_inner,
                    |_, self_field| {
                        subtrees.push(self_field);
                    }
                );
            }
            Self::Bellatrix(self_inner) => {
                map_beacon_state_bellatrix_tree_list_fields_immutable!(
                    &'a _,
                    self_inner,
                    |_, self_field| {
                        subtrees.push(self_field);
                    }
                );
            }
            Self::Capella(self_inner) => {
                map_beacon_state_capella_tree_list_fields_immutable!(
                    &'a _,
                    self_inner,
                    |_, self_field| {
                        subtrees.push(self_field);
                    }
                );
            }
            Self::Deneb(self_inner) => {
                map_beacon_state_deneb_tree_list_fields_immutable!(
                    &'a _,
                    self_inner,
                    |_, self_field| {
                        subtrees.push(self_field);
                    }
                );
            }
            Self::Electra(self_inner) => {
                map_beacon_state_electra_tree_list_fields_immutable!(
                    &'a _,
                    self_inner,
                    |_, self_field| {
                        subtrees.push(self_field);
                    }
                );
            }
            Self::Fulu(self_inner) => {
                map_beacon_state_fulu_tree_list_fields_immutable!(
                    &'a _,
                    self_inner,
                    |_, self_field| {
                        subtrees.push(self_field);
                    }
                );
            }
            Self::Gloas(self_inner) => {
                map_beacon_state_gloas_tree_list_fields_immutable!(
                    &'a _,
                    self_inner,
                    |_, self_field| {
                        subtrees.push(self_field);
                    }
                );
            }
        }
        // if let Ok(current_sc) = self.current_sync_committee() {
        //     subtrees.push(&**current_sc);
        // }
        // if let Ok(next_sc) = self.next_sync_committee() {
        //     subtrees.push(&**next_sc);
        // }

        // for committee_cache in self.committee_caches() {
        //     subtrees.push(&**committee_cache);
        // }

        // FIXME(sproul): more caches

        subtrees
    }

    fn intrinsic_size(&self) -> usize {
        // A close enough approximation
        std::mem::size_of::<Self>()
    }
}

// Implement MemorySize for local element types
impl MemorySize for Eth1Data {
    fn self_pointer(&self) -> usize {
        self as *const _ as usize
    }
    fn subtrees<'a>(&'a self) -> Vec<&'a (dyn MemorySize + 'a)> {
        Vec::new()
    }
    fn intrinsic_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl MemorySize for Validator {
    fn self_pointer(&self) -> usize {
        self as *const _ as usize
    }
    fn subtrees<'a>(&'a self) -> Vec<&'a (dyn MemorySize + 'a)> {
        Vec::new()
    }
    fn intrinsic_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl MemorySize for ParticipationFlags {
    fn self_pointer(&self) -> usize {
        self as *const _ as usize
    }
    fn subtrees<'a>(&'a self) -> Vec<&'a (dyn MemorySize + 'a)> {
        Vec::new()
    }
    fn intrinsic_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl<E: EthSpec> MemorySize for PendingAttestation<E> {
    fn self_pointer(&self) -> usize {
        self as *const _ as usize
    }
    fn subtrees<'a>(&'a self) -> Vec<&'a (dyn MemorySize + 'a)> {
        Vec::new()
    }
    fn intrinsic_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl MemorySize for HistoricalSummary {
    fn self_pointer(&self) -> usize {
        self as *const _ as usize
    }
    fn subtrees(&self) -> Vec<&dyn MemorySize> {
        vec![]
    }
    fn intrinsic_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl MemorySize for PendingDeposit {
    fn self_pointer(&self) -> usize {
        self as *const _ as usize
    }
    fn subtrees(&self) -> Vec<&dyn MemorySize> {
        vec![]
    }
    fn intrinsic_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl MemorySize for PendingPartialWithdrawal {
    fn self_pointer(&self) -> usize {
        self as *const _ as usize
    }
    fn subtrees(&self) -> Vec<&dyn MemorySize> {
        vec![]
    }
    fn intrinsic_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl MemorySize for PendingConsolidation {
    fn self_pointer(&self) -> usize {
        self as *const _ as usize
    }
    fn subtrees(&self) -> Vec<&dyn MemorySize> {
        vec![]
    }
    fn intrinsic_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}
