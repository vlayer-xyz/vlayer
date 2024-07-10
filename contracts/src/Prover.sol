// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface ITraveler {
    function setBlock(uint blockNo) external;
    function setChain(uint chainId, uint blockNo) external;
}

contract Prover {
    // Address generated from first 20-bytes of "vlayer.traveler"'s keccak256.
    ITraveler constant TRAVELER = ITraveler(address(uint160(uint256(keccak256("vlayer.traveler")))));

    function setBlock(uint blockNo) public {
        TRAVELER.setBlock(blockNo);
    }

    function setChain(uint chainId, uint blockNo) public {
        TRAVELER.setChain(chainId, blockNo);
    }
}
