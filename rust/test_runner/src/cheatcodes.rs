use alloy_sol_types::sol;

sol!(
    struct Seal {
        bytes18 lhv;
        bytes19 rhv;
    }
    struct ExecutionCommitment {
        address proverContractAddress;
        bytes4 functionSelector;
        uint256 settleBlockNumber; // Block number at which the commitment was made.
        bytes32 settleBlockHash; // Hash of the block at the specified block number.
    }
    struct Proof {
        uint256 length;
        Seal seal;
        ExecutionCommitment commitment;
    }


    function callProver() external returns (bool);
    function getProof() external returns (Proof);
);
