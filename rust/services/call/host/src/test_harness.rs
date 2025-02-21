use call_engine::HostOutput;
use guest_wrapper::CALL_GUEST_ELF;
use host_utils::ProofMode;
use provider::CachedMultiProvider;
pub use rpc::block_tag_to_block_number;
use rpc::create_multi_provider;
pub use types::ExecutionLocation;

use crate::{BuilderError, Call, Config, Error, Host};

pub mod contracts;
pub mod rpc;
mod types;

pub async fn run(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
) -> Result<HostOutput, Error> {
    run_with_teleport(test_name, call, location).await
}

pub async fn run_with_teleport(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
) -> Result<HostOutput, Error> {
    let multi_provider = create_multi_provider(test_name);
    let host = create_host(multi_provider, location)?;
    let result = host.main(call).await?;

    Ok(result)
}

fn create_host(
    multi_provider: CachedMultiProvider,
    location: &ExecutionLocation,
) -> Result<Host, BuilderError> {
    let config = Config {
        proof_mode: ProofMode::Groth16,
        call_guest_elf: CALL_GUEST_ELF.clone(),
    };
    let block_number =
        block_tag_to_block_number(&multi_provider, location.chain_id, location.block_tag)?;
    let start_exec_location = (location.chain_id, block_number).into();
    Host::try_new(multi_provider, start_exec_location, config)
}
