// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std-1.9.2/src/Test.sol";
import {Proof} from "../Proof.sol";

// 0xe5F6E4A8da66436561059673919648CdEa4e486B
address constant CHEATCODES = address(uint160(uint256(keccak256("vlayer.cheatcodes"))));
uint256 constant VTEST_CHAIN_ID = 31_337;

interface ICheatCodes {
    function callProver() external returns (bool);
    function getProof() external returns (Proof memory);
}

contract VTest is Test {
    constructor() {
        // solhint-disable-next-line reason-string
        require(
            block.chainid == VTEST_CHAIN_ID,
            "Incorrect test chain ID. Make sure you call the tests using `vlayer test` command"
        );
    }

    function callProver() internal {
        ICheatCodes(CHEATCODES).callProver();
    }

    function getProof() internal returns (Proof memory) {
        vm.roll(block.number + 1);
        return ICheatCodes(CHEATCODES).getProof();
    }
}
