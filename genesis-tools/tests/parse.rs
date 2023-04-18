use std::{path::PathBuf, str::FromStr};

use aptos_types::account_address::AccountAddress;
use genesis_tools::{parse_json, convert_types};

#[test]

fn parse_json() {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/sample_end_user_single.json");
    
    dbg!(&p);
    let r = parse_json::parse(p).unwrap();
    dbg!(&r);
    if let Some(acc) = r[0].account {
        dbg!(&acc);
        let acc_str = acc.to_string();
        // let new_acc = AccountAddress::from_str(&acc_str);
        let new_acc = AccountAddress::from_hex_literal(&format!("0x{}",acc_str)).unwrap();
        dbg!(&new_acc);
        let a = convert_types::convert_account(acc);
        dbg!(&a);
    }
    


    // let recovery_json_path = PathBuf::from("tests/fixtures/recovery.json");

    // let recovery = make_recovery_genesis_from_vec_legacy_recovery(recovery_json_path).unwrap();

    // assert_eq!(recovery.len(), 1);

}