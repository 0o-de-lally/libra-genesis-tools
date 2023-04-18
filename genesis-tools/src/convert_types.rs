

use diem_types::account_address::AccountAddress as LegacyAddress;
use aptos_types::account_address::AccountAddress;

pub fn convert_account(acc: LegacyAddress) -> anyhow::Result<AccountAddress>{
  let acc_str = acc.to_string();
  AccountAddress::from_hex_literal(&format!("0x{}",acc_str))
  .map_err(|e| anyhow::anyhow!(e))
}