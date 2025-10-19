use bls::{FixedBytesExtended, Hash256, PublicKeyBytes};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use rand::Rng;
use ssz::Decode;
use std::num::NonZeroUsize;
use store::state_cache::StateCache;

use types::{
    BeaconState, ChainSpec, Epoch, Eth1Data, EthSpec, MainnetEthSpec as E, Slot, Validator,
};

fn build_state(
    spec: &ChainSpec,
    slot: Slot,
    validator_count: usize,
    rng: &mut impl Rng,
) -> BeaconState<E> {
    let genesis_time = 0;
    let eth1_data = Eth1Data::default();
    let mut state = BeaconState::<E>::new(genesis_time, eth1_data, spec);

    for _ in 0..validator_count {
        append_validator(&mut state, rng);
    }

    *state.slot_mut() = slot;
    state.latest_block_header_mut().slot = slot;
    state.apply_pending_mutations().unwrap();
    state
}

fn append_validator(state: &mut BeaconState<E>, mut rng: &mut impl Rng) {
    state
        .balances_mut()
        .push(32_000_000_000 + rng.random_range(1..=1_000_000_000))
        .unwrap();
    if let Ok(inactivity_scores) = state.inactivity_scores_mut() {
        inactivity_scores.push(0).unwrap();
    }
    state
        .validators_mut()
        .push(rand_validator(&mut rng))
        .unwrap();
}

fn rand_validator(rng: &mut impl Rng) -> Validator {
    let mut pubkey = [0u8; 48];
    rng.fill_bytes(&mut pubkey);
    let withdrawal_credentials: [u8; 32] = rng.random();

    Validator {
        pubkey: PublicKeyBytes::from_ssz_bytes(&pubkey).unwrap(),
        withdrawal_credentials: withdrawal_credentials.into(),
        slashed: false,
        effective_balance: 32_000_000_000,
        activation_eligibility_epoch: Epoch::max_value(),
        activation_epoch: Epoch::max_value(),
        exit_epoch: Epoch::max_value(),
        withdrawable_epoch: Epoch::max_value(),
    }
}

pub fn all_benches(c: &mut Criterion) {
    let spec = E::default_spec();
    let mut rng = rand::rng();
    let num_states = 20;
    let validator_count = 1024;

    let states: Vec<(Hash256, Hash256, BeaconState<E>)> = (0..num_states)
        .map(|i| {
            let slot = Slot::new(i as u64);
            let state = build_state(&spec, slot, validator_count, &mut rng);
            let root = Hash256::from_low_u64_le(i as u64 + 1);
            (root, root, state)
        })
        .collect();

    let capacity = NonZeroUsize::new(num_states).unwrap();
    let headroom = NonZeroUsize::new(1).unwrap();
    let hdiff_capacity = NonZeroUsize::new(1).unwrap();

    c.bench_function("state_cache_insert_without_memory_limit", |b| {
        b.iter_batched(
            || StateCache::new(capacity, headroom, hdiff_capacity, usize::MAX),
            |mut cache| {
                for (state_root, block_root, state) in &states {
                    cache.put_state(*state_root, *block_root, state).unwrap();
                }
            },
            BatchSize::SmallInput,
        );
    });

    let low_max_bytes = 1_000_000;
    c.bench_function("state_cache_insert_with_memory_limit", |b| {
        b.iter_batched(
            || StateCache::new(capacity, headroom, hdiff_capacity, low_max_bytes),
            |mut cache| {
                for (state_root, block_root, state) in &states {
                    cache.put_state(*state_root, *block_root, state).unwrap();
                }
                assert!(cache.cached_bytes() <= cache.max_cached_bytes());
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = all_benches
}
criterion_main!(benches);
