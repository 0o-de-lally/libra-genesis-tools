//! keys

use libra_crypto::ed25519::Ed25519PrivateKey;
use libra_sdk::types::account_address::AccountAddress;
use bip39::{Language, Mnemonic, Seed};
use ed25519_dalek_bip32::{DerivationPath, ExtendedSecretKey};
use std::str::FromStr;

use libra_sdk::types::AccountKey;


const COIN_TYPE: u32 = 11259375; // 0L's number per https://github.com/satoshilabs/slips/blob/master/slip-0044.md

#[test]
fn test_keys() {
    let derive_path = &format!("m/44'/{}'/0'/0'/0'", COIN_TYPE);
    let mnemonic_phrase =
        "talent sunset lizard pill fame nuclear spy noodle basket okay critic grow sleep legend hurry pitch blanket clerk impose rough degree sock insane purse";
    let bob = LocalAccount::from_derive_path(derive_path, mnemonic_phrase, 0).unwrap();
    dbg!(&bob.address);
}

#[derive(Debug)]
pub struct LocalAccount {
    /// Address of the account.
    pub address: AccountAddress,
    /// Authentication key of the account.
    key: AccountKey,
    /// Latest known sequence number of the account, it can be different from validator.
    sequence_number: u64,
}

impl LocalAccount {
    pub fn from_derive_path(
        derive_path: &str,
        mnemonic_phrase: &str,
        sequence_number: u64,
    ) -> anyhow::Result<Self> {
        let derive_path = DerivationPath::from_str(derive_path)?;
        let mnemonic = Mnemonic::from_phrase(mnemonic_phrase, Language::English)?;
        // TODO: Make `password` as an optional argument.
        let seed = Seed::new(&mnemonic, "");
        let key = ExtendedSecretKey::from_seed(seed.as_bytes())?
            .derive(&derive_path)?
            .secret_key;
        let key = AccountKey::from(Ed25519PrivateKey::try_from(key.as_bytes().as_ref())?);
        let address = key.authentication_key().derived_address();

        Ok(Self {
            address,
            key,
            sequence_number,
        })
    }
  }
