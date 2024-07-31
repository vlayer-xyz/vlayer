// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {Proof, Seal, ExecutionCommitment} from "../Proof.sol";

// 0xe5F6E4A8da66436561059673919648CdEa4e486B
address constant CHEATCODES = address(uint160(uint256(keccak256("vlayer.cheatcodes"))));

interface ICheatCodes {
    function callProver() external returns (bool);
    function getProof() external returns (Proof memory);
}

contract VTest is Test {
    function callProver() internal {
        ICheatCodes(CHEATCODES).callProver();
    }

    function getProof() internal returns (Proof memory) {
        return ICheatCodes(CHEATCODES).getProof();
    }
}
