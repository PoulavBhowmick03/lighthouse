use crate::beacon_chain::BeaconChainTypes;
use crate::persisted_fork_choice::PersistedForkChoice;
use crate::schema_change::StoreError;
use crate::test_utils::{PersistedBeaconChain, BEACON_CHAIN_DB_KEY, FORK_CHOICE_DB_KEY};
use crate::BeaconForkChoiceStore;
use fork_choice::{ForkChoice, ResetPayloadStatuses};
use slog::Logger;
use ssz::{Decode, Encode};
use ssz_derive::{Decode, Encode};
use std::sync::Arc;
use store::{DBColumn, Error, HotColdDB, KeyValueStoreOp, StoreItem};
use types::{Hash256, Slot};

/// Dummy value to use for the canonical head block root, see below.
pub const DUMMY_CANONICAL_HEAD_BLOCK_ROOT: Hash256 = Hash256::repeat_byte(0xff);

pub fn upgrade_to_v23<T: BeaconChainTypes>(
    db: Arc<HotColdDB<T::EthSpec, T::HotStore, T::ColdStore>>,
    _log: Logger,
) -> Result<Vec<KeyValueStoreOp>, Error> {
    // Set the head-tracker to empty

    let Some(persisted_beacon_chain_v22) =
        db.get_item::<PersistedBeaconChainV22>(&BEACON_CHAIN_DB_KEY)?
    else {
        // If there is no persisted beacon chain, ignore the upgrade
        return Ok(vec![]);
    };

    let persisted_beacon_chain = PersistedBeaconChain {
        genesis_block_root: persisted_beacon_chain_v22.genesis_block_root,
    };

    db.put_item::<PersistedBeaconChain>(&BEACON_CHAIN_DB_KEY, &persisted_beacon_chain)?;

    todo!();
}

pub fn downgrade_from_v23<T: BeaconChainTypes>(
    db: Arc<HotColdDB<T::EthSpec, T::HotStore, T::ColdStore>>,
    log: Logger,
) -> Result<Vec<KeyValueStoreOp>, Error> {
    // recreate head-tracker from the fork-choice

    let Some(persisted_fork_choice) = db.get_item::<PersistedForkChoice>(&FORK_CHOICE_DB_KEY)?
    else {
        // Is it possible for the fork-choice to not exist on a downgrade?
        return Ok(vec![]);
    };

    let Some(persisted_beacon_chain) = db.get_item::<PersistedBeaconChain>(&BEACON_CHAIN_DB_KEY)?
    else {
        // TODO: Is it possible for persisted beacon chain to be missing if the fork choice exists?
        return Ok(vec![]);
    };

    let fc_store =
        BeaconForkChoiceStore::from_persisted(persisted_fork_choice.fork_choice_store, db.clone())
            .map_err(|e| {
                Error::MigrationError(format!(
                    "Error loading fork choise store from persisted: {e:?}"
                ))
            })?;

    // TODO: what value to choose here?
    let reset_payload_statuses = ResetPayloadStatuses::OnlyWithInvalidPayload;
    let mut fork_choice = ForkChoice::from_persisted(
        persisted_fork_choice.fork_choice,
        reset_payload_statuses,
        fc_store,
        &db.spec,
        &log,
    )
    .map_err(|e| {
        Error::MigrationError(format!("Error loading fork choice from persisted: {e:?}"))
    })?;

    // TODO: initialize clock
    let current_slot = Slot::new(0);
    let head_block_root = fork_choice
        .get_head(current_slot, &db.spec)
        .map_err(|e| Error::MigrationError(format!("Error computing get_head: {e:?}")))?;

    let head_proto_block = fork_choice
        .get_block(&head_block_root)
        .ok_or(Error::MigrationError(format!(
            "HeadBlockMissingFromForkChoice({head_block_root:?})"
        )))?;

    let heads = fork_choice
        .proto_array()
        .viable_heads::<T::EthSpec>(head_proto_block.slot);

    let head_roots = heads.iter().map(|node| node.root).collect();
    let head_slots = heads.iter().map(|node| node.slot).collect();

    let persisted_beacon_chain_v22 = PersistedBeaconChainV22 {
        _canonical_head_block_root: DUMMY_CANONICAL_HEAD_BLOCK_ROOT,
        genesis_block_root: persisted_beacon_chain.genesis_block_root,
        ssz_head_tracker: SszHeadTracker {
            roots: head_roots,
            slots: head_slots,
        },
    };

    db.put_item::<PersistedBeaconChainV22>(&BEACON_CHAIN_DB_KEY, &persisted_beacon_chain_v22)?;

    todo!();
}

/// Helper struct that is used to encode/decode the state of the `HeadTracker` as SSZ bytes.
///
/// This is used when persisting the state of the `BeaconChain` to disk.
#[derive(Encode, Decode, Clone)]
pub struct SszHeadTracker {
    roots: Vec<Hash256>,
    slots: Vec<Slot>,
}

#[derive(Clone, Encode, Decode)]
pub struct PersistedBeaconChainV22 {
    /// This value is ignored to resolve the issue described here:
    ///
    /// https://github.com/sigp/lighthouse/pull/1639
    ///
    /// Its removal is tracked here:
    ///
    /// https://github.com/sigp/lighthouse/issues/1784
    pub _canonical_head_block_root: Hash256,
    pub genesis_block_root: Hash256,
    /// DEPRECATED
    pub ssz_head_tracker: SszHeadTracker,
}

impl StoreItem for PersistedBeaconChainV22 {
    fn db_column() -> DBColumn {
        DBColumn::BeaconChain
    }

    fn as_store_bytes(&self) -> Vec<u8> {
        self.as_ssz_bytes()
    }

    fn from_store_bytes(bytes: &[u8]) -> Result<Self, StoreError> {
        Self::from_ssz_bytes(bytes).map_err(Into::into)
    }
}
