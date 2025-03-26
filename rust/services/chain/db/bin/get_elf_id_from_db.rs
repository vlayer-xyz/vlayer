//! This script retrieves chain guest ELF ID from mdbx database (for a given chain ID)
//! Usage: `get_elf_id_from_db <DB_PATH> <CHAIN_ID>`

use std::env;

use alloy_primitives::B256;
use chain_common::ChainProofReceipt;
use chain_db::{ChainDb, Mode};
use risc0_zkvm::{Receipt, sha::Digest};

pub fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let db_path = args
        .get(1)
        .ok_or(anyhow::anyhow!("usage: {} <DB_PATH> <CHAIN_ID>", args[0]))?;
    let chain_id = args
        .get(2)
        .ok_or(anyhow::anyhow!("usage: {} <DB_PATH> <CHAIN_ID>", args[0]))?
        .parse()
        .map_err(|e| anyhow::anyhow!("invalid chain ID: {e:?}"))?;

    let chain_db = ChainDb::mdbx(db_path, Mode::ReadOnly, [])?;
    let (_, _, zk_proof) = chain_db
        .get_chain_info(chain_id)?
        .ok_or(anyhow::anyhow!("no chain info for chain {chain_id}"))?
        .into_parts();
    let receipt: Receipt = ChainProofReceipt::try_from(&zk_proof)?.into();
    let (_, elf_id): (B256, Digest) = receipt.journal.decode()?;

    println!("Chain guest ELF ID: {elf_id:?}");
    Ok(())
}
