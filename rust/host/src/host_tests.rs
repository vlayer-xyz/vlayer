#[cfg(test)]
mod test {

    use crate::host::{Host, HostError};
    use guest_wrapper::GUEST_ELF;
    use risc0_zkvm::ExecutorEnv;

    #[test]
    fn host_prove_invalid_guest_elf() {
        let env = ExecutorEnv::default();
        let invalid_guest_elf = &[];
        let res = Host::prove(env, invalid_guest_elf);

        assert_eq!(res.unwrap_err(), HostError::ElfParseError);
    }

    #[test]
    fn host_prove_invalid_input() {
        let env = ExecutorEnv::default();
        let res = Host::prove(env, GUEST_ELF);

        assert_eq!(res.unwrap_err(), HostError::InvalidInput);
    }

    #[test]
    fn try_new_invalid_rpc_url() {
        let res = Host::try_new("http://localhost:123");

        assert_eq!(res.map(|_| ()).unwrap_err(), HostError::RpcConnectionError);
    }
}
