use ethers::{
    providers::{MockProvider, Provider},
    types::{Block, H256},
};
use provider::{to_eth_block_header, BlockNumber, EvmBlockHeader};
use serde_json::{from_value, json};

fn fake_rpc_block(number: BlockNumber) -> Block<()> {
    // All fields are zeroed out except for the block number
    from_value(json!({
        "number": format!("{:x}", number),
        "baseFeePerGas": "0x0",
        "miner": "0x0000000000000000000000000000000000000000",
        "hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "nonce": "0x0000000000000000",
        "sealFields": [],
        "sha3Uncles": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "transactionsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "receiptsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "stateRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "difficulty": "0x0",
        "totalDifficulty": "0x0",
        "extraData": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "size": "0x0",
        "gasLimit": "0x0",
        "minGasPrice": "0x0",
        "gasUsed": "0x0",
        "timestamp": "0x0",
        "transactions": [],
        "uncles": []
    })).unwrap()
}

fn fake_rpc_block_with_correct_parent_hash(number: BlockNumber) -> Block<()> {
    let mut rpc_block = fake_rpc_block(number);
    if number > 0 {
        // Parent hash is only checked when prepending and you can't prepend to the genesis block
        let parent_block = fake_block(number - 1 /* underflows for genesis */);
        rpc_block.parent_hash = H256::from_slice(parent_block.hash_slow().as_slice());
    }
    rpc_block
}

pub fn fake_block(number: BlockNumber) -> Box<dyn EvmBlockHeader> {
    let rpc_block = fake_rpc_block(number);
    let block = to_eth_block_header(rpc_block).expect("could not convert block");
    Box::new(block)
}

pub fn mock_provider(
    block_numbers: impl IntoIterator<Item = BlockNumber>,
) -> Provider<MockProvider> {
    let (provider, mock) = Provider::mocked();
    // Mock provider is a stack (LIFO). Therefore we need to push the mock values in reverse order
    let reverse_block_numbers = Vec::from_iter(block_numbers).into_iter().rev();
    for block_number in reverse_block_numbers {
        mock.push(fake_rpc_block_with_correct_parent_hash(block_number))
            .expect("could not push block");
    }
    provider
}
