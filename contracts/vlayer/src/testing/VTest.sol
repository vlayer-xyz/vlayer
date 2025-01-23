// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

// solhint-disable-next-line no-global-import
import "forge-std-1.9.4/src/Test.sol"; // reexports foundry test modules

import {UnverifiedEmail} from "../EmailProof.sol";
import {Proof} from "../Proof.sol";
import {EmailTestUtils} from "./libraries/EmailTestUtils.sol";

// 0xe5F6E4A8da66436561059673919648CdEa4e486B
address constant CHEATCODES = address(uint160(uint256(keccak256("vlayer.cheatcodes"))));
uint256 constant VTEST_CHAIN_ID = 30_1337;

interface ICheatCodes {
    function callProver() external returns (bool);
    function getProof() external returns (Proof memory);
    function preverifyEmail(string memory email) external view returns (UnverifiedEmail memory);
}

contract VTest is Test {
    constructor() {
        // solhint-disable-next-line reason-string
        require(
            block.chainid == VTEST_CHAIN_ID,
            "Incorrect test chain ID. Make sure you call the tests using `vlayer test` command"
        );
    }

    // solhint-disable-next-line no-empty-blocks
    function setUp() internal {
        // setUp is not allowed in VTest tests
    }

    function callProver() internal {
        ICheatCodes(CHEATCODES).callProver();
    }

    function getProof() internal returns (Proof memory) {
        vm.roll(block.number + 1);
        return ICheatCodes(CHEATCODES).getProof();
    }

    function preverifyEmail(string memory email) internal view returns (UnverifiedEmail memory) {
        return EmailTestUtils.preverifyEmail(email);
    }
}
