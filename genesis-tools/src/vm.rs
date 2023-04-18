
use libra_vm_genesis::{
  Validator,
  verify_genesis_write_set,
  publish_framework,
  genesis_context::GenesisStateView,
  emit_new_block_and_epoch_event,
  set_genesis_end,
  allow_core_resources_to_set_version,
  create_and_initialize_validators,
  initialize_on_chain_governance,
  initialize_aptos_coin,
  initialize_core_resources_and_aptos_coin,
  initialize_features,
  initialize,
  validate_genesis_config,
  GenesisConfiguration,
  default_gas_schedule,
  mainnet_genesis_config,
  GENESIS_KEYPAIR,

};
use aptos_crypto::{
    ed25519::{Ed25519PublicKey},
    HashValue,
};
use aptos_framework::{self, ReleaseBundle};
use aptos_gas::{
    AbstractValueSizeGasParameters, ChangeSetConfigs,
    NativeGasParameters, LATEST_GAS_FEATURE_VERSION,
};
use aptos_types::{
    chain_id::ChainId,
    on_chain_config::{
        Features, GasScheduleV2, OnChainConsensusConfig, TimedFeatures,
    },
    transaction::{ChangeSet},
};
use aptos_vm::{
    data_cache::AsMoveResolver,
    move_vm_ext::{MoveVmExt, SessionId},
};

pub fn libra_mainnet_genesis(
    validators: Vec<Validator>,
) -> anyhow::Result<(ChangeSet, Vec<Validator>)> {
    let genesis = encode_libra_recovery_genesis_change_set(
        &GENESIS_KEYPAIR.1,
        &validators,
        aptos_framework::testnet_release_bundle(),
        ChainId::test(),
        &mainnet_genesis_config(),
        &OnChainConsensusConfig::default(),
        &default_gas_schedule(),
    );
    Ok((genesis, validators))
}

/// Generates a genesis using the recovery file for hard forks.
pub fn encode_libra_recovery_genesis_change_set(
    core_resources_key: &Ed25519PublicKey,
    validators: &[Validator],
    framework: &ReleaseBundle,
    chain_id: ChainId,
    genesis_config: &GenesisConfiguration,
    consensus_config: &OnChainConsensusConfig,
    gas_schedule: &GasScheduleV2,
) -> ChangeSet {
    validate_genesis_config(genesis_config);

    // Create a Move VM session so we can invoke on-chain genesis intializations.
    let mut state_view = GenesisStateView::new();
    for (module_bytes, module) in framework.code_and_compiled_modules() {
        state_view.add_module(&module.self_id(), module_bytes);
    }
    let data_cache = state_view.as_move_resolver();
    let move_vm = MoveVmExt::new(
        NativeGasParameters::zeros(),
        AbstractValueSizeGasParameters::zeros(),
        LATEST_GAS_FEATURE_VERSION,
        ChainId::test().id(),
        Features::default(),
        TimedFeatures::enable_all(),
    )
    .unwrap();
    let id1 = HashValue::zero();
    let mut session = move_vm.new_session(&data_cache, SessionId::genesis(id1));

    // On-chain genesis process.
    initialize(
        &mut session,
        chain_id,
        genesis_config,
        consensus_config,
        gas_schedule,
    );
    initialize_features(&mut session);
    if genesis_config.is_test {
        initialize_core_resources_and_aptos_coin(&mut session, core_resources_key);
    } else {
        initialize_aptos_coin(&mut session);
    }
    initialize_on_chain_governance(&mut session, genesis_config);
    create_and_initialize_validators(&mut session, validators);
    if genesis_config.is_test {
        allow_core_resources_to_set_version(&mut session);
    }
    set_genesis_end(&mut session);

    // Reconfiguration should happen after all on-chain invocations.
    emit_new_block_and_epoch_event(&mut session);

    let cs1 = session
        .finish(
            &mut (),
            &ChangeSetConfigs::unlimited_at_gas_feature_version(LATEST_GAS_FEATURE_VERSION),
        )
        .unwrap();

    let state_view = GenesisStateView::new();
    let data_cache = state_view.as_move_resolver();

    // Publish the framework, using a different session id, in case both scripts creates tables
    let mut id2_arr = [0u8; 32];
    id2_arr[31] = 1;
    let id2 = HashValue::new(id2_arr);
    let mut session = move_vm.new_session(&data_cache, SessionId::genesis(id2));
    publish_framework(&mut session, framework);
    let cs2 = session
        .finish(
            &mut (),
            &ChangeSetConfigs::unlimited_at_gas_feature_version(LATEST_GAS_FEATURE_VERSION),
        )
        .unwrap();

    let change_set_ext = cs1.squash(cs2).unwrap();

    let (delta_change_set, change_set) = change_set_ext.into_inner();

    // Publishing stdlib should not produce any deltas around aggregators and map to write ops and
    // not deltas. The second session only publishes the framework module bundle, which should not
    // produce deltas either.
    assert!(
        delta_change_set.is_empty(),
        "non-empty delta change set in genesis"
    );

    assert!(!change_set
        .write_set()
        .iter()
        .any(|(_, op)| op.is_deletion()));
    verify_genesis_write_set(change_set.events());
    change_set
}