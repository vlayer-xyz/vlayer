#[cfg(test)]
mod test {

    use crate::host::{prove, HostError};
    use guest_wrapper::GUEST_ELF;
    use risc0_zkvm::ExecutorEnv;

    #[test]
    fn host_prove_invalid_guest_elf() {
        let env = ExecutorEnv::default();
        let invalid_guest_elf = &[];
        let res = prove(env, invalid_guest_elf);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Elf parse error: Could not read bytes in range [0x0, 0x10)"
        ));
    }

    #[test]
    fn host_prove_invalid_input() {
        let env = ExecutorEnv::default();
        let res = prove(env, GUEST_ELF);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Guest panicked: called `Result::unwrap()` on an `Err` value: DeserializeUnexpectedEnd"
        ));
    }
}
