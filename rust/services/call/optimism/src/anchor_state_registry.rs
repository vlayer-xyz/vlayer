use alloy_primitives::{Address, B256, BlockNumber};
use alloy_sol_types::{SolCall, sol};
use anyhow::anyhow;
use call_common::RevmDB;
use derive_new::new;
use revm::{
    Evm,
    primitives::{ExecutionResult, ResultAndState, TxEnv},
};

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] anyhow::Error);
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, new, PartialEq, Eq)]
pub struct L2Commitment {
    pub output_hash: B256,
    pub block_number: BlockNumber,
}

#[derive(Clone, Debug, new)]
pub struct AnchorStateRegistry<D: RevmDB> {
    address: Address,
    db: D,
}

fn evm_call<C: SolCall>(db: impl RevmDB, to: Address, call: C) -> Result<C::Return> {
    let tx_env = TxEnv {
        transact_to: to.into(),
        data: call.abi_encode().into(),
        ..Default::default()
    };
    let mut evm = Evm::builder().with_ref_db(db).with_tx_env(tx_env).build();

    let ResultAndState { result, .. } = evm.transact_preverified().map_err(anyhow::Error::new)?;
    let ExecutionResult::Success { output, .. } = result else {
        return Err(Error(anyhow!("Failed to get latest confirmed L2 commitment")));
    };
    let result = C::abi_decode_returns(output.data(), true)
        .map_err(|_| Error(anyhow!("Failed to decode latest confirmed L2 commitment")))?;
    Ok(result)
}

sol! {
    struct OutputRoot {
        bytes32 output_hash;
        uint256 block_number;
    }
    function anchors(uint32) public view returns (OutputRoot);
}

impl<D: RevmDB> AnchorStateRegistry<D> {
    pub fn get_latest_confirmed_l2_commitment(&self) -> Result<L2Commitment> {
        let anchorsReturn {
            _0:
                OutputRoot {
                    output_hash,
                    block_number,
                },
        } = evm_call(&self.db, self.address, anchorsCall { _0: 1 })?;

        Ok(L2Commitment {
            output_hash,
            block_number: block_number.try_into().map_err(|_| {
                Error(anyhow!("Block number returned from getAnchorRoot overflows u64"))
            })?,
        })
    }
}
