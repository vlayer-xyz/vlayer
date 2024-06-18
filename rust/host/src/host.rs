use anyhow::{anyhow, Error};

use ethers_providers::{Http, Provider, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, ProveInfo};
use vlayer_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::{call::evm_call, CallTxData},
    ethereum::EthEvmEnv,
    guest_input::{Call, GuestInput},
    host::{db::ProofDb, provider::EthersProvider},
};

pub type EthersClient = Provider<RetryClient<Http>>;

pub struct Host {
    env: EthEvmEnv<ProofDb<EthersProvider<EthersClient>>>,
}

#[derive(Debug)]
pub enum HostError {
    ProveFailed(Error),
}

impl std::fmt::Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProveFailed(err) => write!(f, "HostError: {}", err),
        }
    }
}

impl Host {
    pub fn try_new() -> anyhow::Result<Self> {
        let env = EthEvmEnv::from_rpc("http://localhost:8545", None)?
            .with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC)?;
        Ok(Host { env })
    }

    pub fn run(mut self, call_tx_data: CallTxData<()>) -> anyhow::Result<Vec<u8>> {
        let CallTxData {
            caller, to, data, ..
        } = call_tx_data.clone();
        let _returns = evm_call(call_tx_data, &mut self.env)?;

        let evm_input = self.env.into_input()?;
        let input = GuestInput {
            evm_input,
            call: Call { caller, to, data },
        };
        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .unwrap();

        let prove_info = Host::prove(env).map_err(|err| anyhow!(err))?;

        Ok(prove_info.receipt.journal.bytes)
    }

    fn prove(env: ExecutorEnv) -> Result<ProveInfo, HostError> {
        let prover = default_prover();
        prover.prove(env, GUEST_ELF).map_err(HostError::ProveFailed)
    }
}
