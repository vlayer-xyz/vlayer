use std::{collections::HashMap, rc::Rc};

use alloy_primitives::Sealable;
use derive_more::{AsMut, IntoIterator};
use vlayer_engine::{
    ethereum::EthBlockHeader,
    evm::env::{EvmEnv, ExecutionLocation, MultiEvmEnv},
};

use crate::{
    db::proof::ProofDb, host::HostError, multiprovider::MultiProvider, provider::Provider,
};

// This ugliness with P and M is temporary.
// We need to constraint P to have a ETH Header type, but associated_type_bounds is not stable yet.
// This will be fixed when we start using dyn for headers. Only M will be needed.
#[derive(AsMut, IntoIterator)]
pub struct HostMultiEvmEnv<P: Provider, M>
where
    P: Provider<Header = EthBlockHeader>,
    M: MultiProvider<Provider = P>,
{
    #[as_mut]
    #[into_iterator(owned)]
    pub envs: MultiEvmEnv<ProofDb<Rc<M::Provider>>, <M::Provider as Provider>::Header>,
    multi_provider: M,
}

impl<P, M> HostMultiEvmEnv<P, M>
where
    P: Provider<Header = EthBlockHeader>,
    M: MultiProvider<Provider = P>,
{
    pub fn new(multi_provider: M) -> Self {
        Self {
            envs: HashMap::new(),
            multi_provider,
        }
    }

    pub fn ensure_vm_exists(&mut self, location: ExecutionLocation) -> Result<(), HostError> {
        #[allow(clippy::map_entry)]
        // Borrow checker doesn't allow us to create VM while we are operating on the map
        if !self.envs.contains_key(&location) {
            let vm = self.create_vm(location)?;
            self.envs.insert(location, vm);
        }
        Ok(())
    }

    fn create_vm(
        &mut self,
        location: ExecutionLocation,
    ) -> Result<EvmEnv<ProofDb<Rc<P>>, P::Header>, HostError> {
        let provider = self.multi_provider.get(location.chain_id)?;
        let start_block_number = location.block_number;
        let header = provider
            .get_block_header(start_block_number)
            .map_err(|err| HostError::Provider(err.to_string()))?
            .ok_or(HostError::BlockNotFound(start_block_number))?;

        let db = ProofDb::new(provider, start_block_number);
        let chain_spec = location.chain_id.try_into()?;

        let mut env = EvmEnv::new(db, header.seal_slow());
        env.with_chain_spec(&chain_spec)?;
        Ok(env)
    }
}
