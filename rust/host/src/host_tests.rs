#[cfg(test)]
mod test {

    use crate::host::{config::HostConfig, error::HostError, Host};
    use crate::provider::{EthersClient, EthersProvider};
    use guest_wrapper::GUEST_ELF;
    use risc0_zkvm::ExecutorEnv;
    use vlayer_engine::config::MAINNET_ID;
    use vlayer_engine::engine::EngineError;
    use vlayer_engine::evm::env::location::ExecutionLocation;
    use vlayer_engine::io::Call;

    #[test]
    fn host_prove_invalid_guest_elf() {
        let env = ExecutorEnv::default();
        let invalid_guest_elf = &[];
        let res = Host::<EthersProvider<EthersClient>>::prove(env, invalid_guest_elf);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Elf parse error: Could not read bytes in range [0x0, 0x10)"
        ));
    }

    #[test]
    fn host_prove_invalid_input() {
        let env = ExecutorEnv::default();
        let res = Host::<EthersProvider<EthersClient>>::prove(env, GUEST_ELF);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Guest panicked: called `Result::unwrap()` on an `Err` value: DeserializeUnexpectedEnd"
        ));
    }

    #[test]
    fn try_new_invalid_rpc_url() -> anyhow::Result<()> {
        let execution_location = ExecutionLocation::new(0, MAINNET_ID);
        let res = Host::try_new(HostConfig::new("http://localhost:123", execution_location))?
            .run(Call::default());

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Engine(EngineError::EvmEnv(ref msg)) if msg.to_string().contains(
                "(http://localhost:123/): error trying to connect: tcp connect error: Connection refused"
            )
        ));

        Ok(())
    }
}
