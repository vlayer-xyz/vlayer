// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface ITraveler {
    function setBlock(uint blockNo) external;
    function setChain(uint chainId, uint blockNo) external;
}

contract Prover {
    // Address generated from first 20-bytes of "vlayer.traveler"'s keccak256.
    ITraveler constant TRAVELER =
        ITraveler(0x2AE215Ce9FDe588aDfdEa92976dC9AA45AA006A0);

    function setBlock(uint blockNo) public {
        TRAVELER.setBlock(blockNo);
    }

    function setChain(uint chainId, uint blockNo) public {
        TRAVELER.setChain(chainId, blockNo);
    }
}
