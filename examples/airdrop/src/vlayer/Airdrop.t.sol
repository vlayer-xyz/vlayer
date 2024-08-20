// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "vlayer/testing/VTest.sol";
import "./NftOwnershipProver.sol";

contract AirdropTest is VTest {
    function test() public {
        NftOwnershipProver prover = new NftOwnershipProver();
        vm.roll(2);
        callProver();
        prover.main(0xaAa2DA255DF9Ee74C7075bCB6D81f97940908A5D);
    }
}