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

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Elf parse error: Could not read bytes in range [0x0, 0x10)"
        ));
    }

    #[test]
    fn host_prove_invalid_input() {
        let env = ExecutorEnv::default();
        let res = Host::prove(env, GUEST_ELF);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Guest panicked: called `Result::unwrap()` on an `Err` value: DeserializeUnexpectedEnd"
        ));
    }

    #[test]
    fn try_new_invalid_rpc_url() {
        let res = Host::try_new("http://localhost:123");

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Provider(ref msg) if msg.to_string().contains("error sending request for url (http://localhost:123/): error trying to connect: tcp connect error: Connection refused")
        ));
    }
}
