use bytes::Bytes;
use call_engine::Input;
use host_utils::{proving, ProofMode, Prover as Risc0Prover};
use risc0_zkvm::{serde::Serializer, ExecutorEnv, ProveInfo};
use tracing::instrument;

#[derive(Debug, Clone, Default)]
pub struct Prover {
    prover: Risc0Prover,
    guest_elf: Bytes,
}

impl Prover {
    pub fn try_new(
        proof_mode: ProofMode,
        guest_elf: impl AsRef<Bytes>,
    ) -> Result<Self, crate::BuilderError> {
        Ok(Self {
            prover: Risc0Prover::try_new(proof_mode)?,
            guest_elf: guest_elf.as_ref().clone(), // Bytes is cheap to clone
        })
    }

    /// Wrapper around Risc0Prover which specifies the call guest ELF
    #[instrument(skip_all)]
    pub fn prove(&self, input: &Input) -> proving::Result<ProveInfo> {
        let executor_env = build_executor_env(input)?;
        Ok(self.prover.prove(executor_env, &self.guest_elf)?)
    }
}

fn to_vec<T>(value: &T) -> Vec<u32>
where
    T: serde::Serialize + ?Sized,
{
    // Use the in-memory size of the value as a guess for the length
    // of the serialized value.
    let mut vec: Vec<u32> = Vec::with_capacity(core::mem::size_of_val(value));
    let mut serializer = Serializer::new(&mut vec);
    value.serialize(&mut serializer).unwrap();
    vec
}

fn dump_input_to_file(input: &Input) {
    use byteorder::{LittleEndian, WriteBytesExt};

    let as_input = to_vec(&input);
    let mut res: Vec<u8> = Vec::new();
    for &x in &as_input {
        let _ = res.write_u32::<LittleEndian>(x);
    }

    std::fs::write("input.dump", res).unwrap();
}

fn build_executor_env(input: &Input) -> anyhow::Result<ExecutorEnv<'static>> {
    let dump = to_vec(&input);
    tracing::info!("input = {dump:#?}");
    dump_input_to_file(input);
    input
        .chain_proofs
        .values()
        .try_fold(ExecutorEnv::builder(), |mut builder, (_, proof)| {
            builder.add_assumption(proof.receipt.clone());
            Ok::<_, anyhow::Error>(builder)
        })?
        .write(&input)?
        .build()
}

#[cfg(test)]
mod tests {
    use guest_wrapper::CALL_GUEST_ELF;

    use super::*;

    #[test]
    fn invalid_input() {
        let res = Prover::try_new(ProofMode::Fake, &CALL_GUEST_ELF)
            .unwrap()
            .prove(&Input::default());

        assert_eq!(
            res.map(|_| ()).unwrap_err().to_string(),
            "Prover: Guest panicked: travel call verification failed: Teleport(Conversion(UnsupportedChainId(0)))"
        );
    }
}
