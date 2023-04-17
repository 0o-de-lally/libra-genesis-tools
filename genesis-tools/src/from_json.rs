use anyhow::Error;
use std::path::PathBuf;

/// Make a recovery genesis blob
pub fn make_recovery_genesis_from_vec_legacy_recovery(
    recovery: &[LegacyRecovery],
    genesis_vals: Vec<AccountAddress>,
    genesis_blob_path: PathBuf,
    append_user_accounts: bool,
) -> Result<Transaction, Error> {
    // get consensus accounts
    // let all_validator_configs = recover_validator_configs(recovery)?;

    // check the validators that are joining genesis actually have legacy data
    let count = all_validator_configs.vals
    .iter()
    .filter(
      |v| {
        genesis_vals.contains(&v.val_account)
      }
    )
    .count();

    if count == 0 {
      anyhow::bail!("no val configs found for genesis set");
    }

    // we use the vm-genesis to properly migrate EVERY validator account.
    // then we select a subset which will be the validators of the first epoch.
    // let genesis_changeset_with_validators = get_baseline_genesis_change_set(all_validator_configs, &genesis_vals)?;

    // let recovery_changeset = encode_recovery_genesis_changeset(
    //   &all_validator_configs.vals,
    //   &all_validator_configs.opers,
    //   &genesis_vals,
    //   1, // mainnet
    //   append_user_accounts,
    //   recovery, // TODO: turn this into an option type
    // )?;

    // For a real upgrade or fork, we want to include all user accounts.
    // this is the default.
    // Otherwise, we might need to just collect the validator accounts
    // for debugging or other test purposes.
    // let expected_len_all_users = recovery.len() as u64;

    // let gen_tx = if append_user_accounts {
    //     // append further writeset to genesis
    //     append_genesis(
    //       genesis_changeset_with_validators,
    //       recovery,
    //       expected_len_all_users
    //     )?
    // } else {
    let gen_tx = Transaction::GenesisTransaction(WriteSetPayload::Direct(recovery_changeset));
    // };
    // save genesis
    // save_genesis(&gen_tx, genesis_blob_path)?;

    Ok(gen_tx)
}