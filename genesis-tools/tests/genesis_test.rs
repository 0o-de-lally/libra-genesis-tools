use genesis_tools::vm::libra_mainnet_genesis;
use libra_vm_genesis::TestValidator;

#[test]
fn test_genesis() {
    // avoid error stake too low: 0x10002
    let test_validators = TestValidator::new_test_set(Some(6), Some(100_000_000_000_000));

    let vec_vals = vec![test_validators[0].data.clone()];
    // dbg!(&vec_vals);
    let (recovery_changeset, _) = libra_mainnet_genesis(vec_vals).unwrap();

}


