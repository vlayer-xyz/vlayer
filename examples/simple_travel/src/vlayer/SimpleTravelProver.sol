// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";

interface IExample {
    function example() external pure returns (uint256);
}

contract SimpleTravelProver is Prover {
    function aroundTheWorld() public returns (uint256) {
        // Important: the address of otherChainContract depends on when it was deployed on anvil 1.
        address otherChainContract = 0x5FbDB2315678afecb367f032d93F642f64180aa3;
        uint mainnetId = 1;
        uint blockNo = 1;

        setChain(mainnetId, blockNo);
        return IExample(otherChainContract).example();
    }
}
