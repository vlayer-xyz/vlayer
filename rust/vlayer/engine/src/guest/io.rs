use alloy_primitives::Address;
use alloy_rlp::Decodable;
use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use risc0_zkvm::Journal;
use serde::{Deserialize, Serialize};

use crate::ethereum::EthEvmInput;
use crate::SolCommitment;

#[derive(Deserialize, Serialize, Debug)]
pub struct Input {
    pub evm_input: EthEvmInput,
    pub call: Call,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Call {
    pub caller: Address,
    pub to: Address,
    pub data: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug, RlpEncodable, RlpDecodable)]
pub struct Output {
    pub execution_commitment: SolCommitment,
    pub evm_call_result: Vec<u8>,
}

impl From<Journal> for Output {
    fn from(value: Journal) -> Self {
        let rlp_output: Vec<u8> = value
            .decode()
            .expect("Could not decode Journal into Vec<u8>");

        Output::decode(&mut rlp_output.as_slice()).expect("Could not decode Vec<u8> to Output")
    }
}
