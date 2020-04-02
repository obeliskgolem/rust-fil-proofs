use std::path::PathBuf;

use anyhow::Result;
use storage_proofs::parameter_cache::{self, CacheableParameters};
use storage_proofs::porep::stacked::{StackedCircuit, StackedCompound};

use crate::constants::{DefaultPieceHasher, DefaultTreeHasher, DefaultBinaryTree};
use crate::types::*;

#[derive(Clone, Copy, Debug)]
pub struct PoRepConfig {
    pub sector_size: SectorSize,
    pub partitions: PoRepProofPartitions,
}

impl From<PoRepConfig> for PaddedBytesAmount {
    fn from(x: PoRepConfig) -> Self {
        let PoRepConfig { sector_size, .. } = x;
        PaddedBytesAmount::from(sector_size)
    }
}

impl From<PoRepConfig> for UnpaddedBytesAmount {
    fn from(x: PoRepConfig) -> Self {
        let PoRepConfig { sector_size, .. } = x;
        PaddedBytesAmount::from(sector_size).into()
    }
}

impl From<PoRepConfig> for PoRepProofPartitions {
    fn from(x: PoRepConfig) -> Self {
        let PoRepConfig { partitions, .. } = x;
        partitions
    }
}

impl From<PoRepConfig> for SectorSize {
    fn from(cfg: PoRepConfig) -> Self {
        let PoRepConfig { sector_size, .. } = cfg;
        sector_size
    }
}

impl PoRepConfig {
    /// Returns the cache identifier as used by `storage-proofs::paramater_cache`.
    pub fn get_cache_identifier(&self) -> Result<String> {
        let params =
            crate::parameters::public_params(self.sector_size.into(), self.partitions.into())?;

        Ok(
            <StackedCompound<DefaultBinaryTree, DefaultPieceHasher> as CacheableParameters<
                StackedCircuit<DefaultBinaryTree, DefaultPieceHasher>,
                _,
            >>::cache_identifier(&params),
        )
    }

    pub fn get_cache_metadata_path(&self) -> Result<PathBuf> {
        let id = self.get_cache_identifier()?;
        Ok(parameter_cache::parameter_cache_metadata_path(&id))
    }

    pub fn get_cache_verifying_key_path(&self) -> Result<PathBuf> {
        let id = self.get_cache_identifier()?;
        Ok(parameter_cache::parameter_cache_verifying_key_path(&id))
    }

    pub fn get_cache_params_path(&self) -> Result<PathBuf> {
        let id = self.get_cache_identifier()?;
        Ok(parameter_cache::parameter_cache_params_path(&id))
    }
}
