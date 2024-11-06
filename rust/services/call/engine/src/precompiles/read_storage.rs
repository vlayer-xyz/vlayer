use std::sync::Arc;

use alloy_primitives::{keccak256, Bytes, U256};
use alloy_sol_types::SolValue;
use revm::{
    precompile::{PrecompileOutput, PrecompileResult},
    primitives::address,
    ContextPrecompile, ContextStatefulPrecompile, Database, InnerEvmContext,
};

pub struct StoragePrecompile;

impl StoragePrecompile {
    pub const fn new() -> Self {
        Self {}
    }

    /// Create a new stateful fatal precompile
    pub fn new_precompile<D: Database>() -> ContextPrecompile<D> {
        ContextPrecompile::ContextStateful(Arc::new(Self::new()))
    }
}

fn get_balance_slot(addr: alloy_primitives::Address) -> U256 {
    let base_slot = U256::from(0);
    let mut key = addr.abi_encode();
    key.extend(base_slot.abi_encode());
    keccak256(key).into()
}

impl<D: Database> ContextStatefulPrecompile<D> for StoragePrecompile {
    fn call(
        &self,
        _input: &Bytes,
        _gas_limit: u64,
        context: &mut InnerEvmContext<D>,
    ) -> PrecompileResult {
        println!(
            "{:?}",
            &context
                .db
                .basic(address!("5FC8d32690cc91D4c39d9d3abcBD16989F875707"))
                .map_err(|_| "Error".to_string())
        );
        get_balance_slot(address!("BFF7D6bA1201304aF302f12265CfA435539D5502"));
        println!(
            "{:?}",
            &context
                .db
                .storage(
                    address!("5FC8d32690cc91D4c39d9d3abcBD16989F875707"),
                    get_balance_slot(address!("BFF7D6bA1201304aF302f12265CfA435539D5502"))
                )
                .map_err(|_| "Error".to_string())
        );
        Ok(PrecompileOutput::new(10, Bytes::new()))
    }
}

pub(super) fn stateful_precompile<D: Database>() -> ContextPrecompile<D> {
    StoragePrecompile::new_precompile::<D>()
}
