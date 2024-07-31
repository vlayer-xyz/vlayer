use alloy_sol_types::private::Address;
use alloy_sol_types::sol;
use forge::revm::primitives::address;

pub const CHEATCODE_CALL_ADDR: Address = address!("e5F6E4A8da66436561059673919648CdEa4e486B");

sol!(
    #[derive(Default)]
    struct Seal {
        bytes18 lhv;
        bytes19 rhv;
    }
    #[derive(Default)]
    struct ExecutionCommitment {
        address proverContractAddress;
        bytes4 functionSelector;
        uint256 settleBlockNumber; // Block number at which the commitment was made.
        bytes32 settleBlockHash; // Hash of the block at the specified block number.
    }
    #[derive(Default)]
    struct Proof {
        uint256 length;
        Seal seal;
        ExecutionCommitment commitment;
    }


    function callProver() external returns (bool);
    function getProof() external returns (Proof memory);
);
