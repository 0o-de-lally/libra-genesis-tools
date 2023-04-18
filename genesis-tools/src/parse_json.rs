use anyhow::Error;
use std::path::PathBuf;
use ol_types::legacy_recovery::{LegacyRecovery, self};


/// Make a recovery genesis blob
pub fn make_recovery_genesis_from_vec_legacy_recovery(
    recovery_json_path: PathBuf,
) -> Result<Vec<LegacyRecovery>, Error> {
  Ok(legacy_recovery::read_from_recovery_file(&recovery_json_path))
}