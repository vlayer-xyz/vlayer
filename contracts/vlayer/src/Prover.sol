// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof, ProofLib} from "./Proof.sol";

interface ITraveler {
    // These functions need to return something because otherwise Solidity compiler won't generate CALL opcode when they're called.
    function setBlock(uint256 blockNo) external returns (bool);

    function setChain(uint256 chainId, uint256 blockNo) external returns (bool);
}

contract Prover {
    // Address generated from first 20-bytes of "vlayer.traveler"'s keccak256.
    // 0x76dc9aa45aa006a0f63942d8f9f21bd4537972a3
    ITraveler private constant TRAVELER = ITraveler(address(uint160(uint256(keccak256("vlayer.traveler")))));

    function setBlock(uint256 blockNo) public {
        require(TRAVELER.setBlock(blockNo), "Failed cheatcode invocation");
    }

    function setChain(uint256 chainId, uint256 blockNo) public {
        require(TRAVELER.setChain(chainId, blockNo), "Failed cheatcode invocation");
    }

    function proof() public pure returns (Proof memory) {
        return ProofLib.emptyProof();
    }
}
