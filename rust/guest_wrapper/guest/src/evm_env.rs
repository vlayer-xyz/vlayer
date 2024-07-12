use derive_more::AsMut;
use vlayer_engine::{
    engine::EngineError,
    evm::{
        block_header::EvmBlockHeader,
        env::{EvmEnv, ExecutionLocation, InnerMultiEvmEnv, MultiEvmEnv},
    },
};

#[derive(AsMut)]
pub struct GuestMultiEvmEnv<D, H>(pub InnerMultiEvmEnv<D, H>);

impl<D, H: EvmBlockHeader> MultiEvmEnv<D, H> for GuestMultiEvmEnv<D, H> {
    fn get_mut(&mut self, location: &ExecutionLocation) -> Result<&mut EvmEnv<D, H>, EngineError> {
        self.as_mut()
            .get_mut(&location)
            .ok_or(EngineError::EvmNotFound(*location))
    }
}
