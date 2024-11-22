use strum::{Display, EnumString};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum ProofMode {
    #[default]
    Fake,
    Groth16,
    Succinct,
}
