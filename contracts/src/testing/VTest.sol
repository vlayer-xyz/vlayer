// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {Proof, Seal, ExecutionCommitment} from "../Proof.sol";

address constant CHEATCODES = address(uint160(uint256(keccak256("vlayer.cheatcodes")))); // 0xe5F6E4A8da66436561059673919648CdEa4e486B

interface ICheatCodes {
    function startProof() external returns (bool);
    function endProof() external returns (bool);
}

contract VTest is Test {
    function startProof() internal {
        ICheatCodes(CHEATCODES).startProof();
    }

    function endProof() internal returns (Proof memory) {
        ICheatCodes(CHEATCODES).endProof();
        return Proof(0, Seal(0, 0), ExecutionCommitment(address(0), bytes4(0), 0, bytes32(0)));
    }
}
