#[derive(Debug, Default, Copy, Clone)]
pub enum ProofMode {
    #[default]
    Fake,
    Groth16,
}
