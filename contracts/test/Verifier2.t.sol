// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std-1.9.2/src/Test.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";

import {IRiscZeroVerifier, Receipt, VerificationFailed} from "risc0-ethereum-1.0.0/src/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum-1.0.0/src/test/RiscZeroMockVerifier.sol";

import {ExecutionCommitment} from "../src/ExecutionCommitment.sol";
import {Proof} from "../src/Proof.sol";

import {FakeProofVerifier} from "../src/proof_verifier/FakeProofVerifier.sol";
import {Verifier} from "../src/Verifier.sol";
import {Verifier2, Proof2} from "../src/Verifier2.sol";

contract Verifier2Test is Test {
    Verifier2 verifier = new Verifier2();
    TestHelpers helpers = new TestHelpers();

    ExecutionCommitment commitment;

    function setUp() public {
        vm.roll(100); // have some historical blocks

        commitment = ExecutionCommitment(
            address(address(1337)), bytes4(uint32(420)), block.number - 1, blockhash(block.number - 1)
        );
    }

    function test_calldataWithDynamicCalls() public view {
        (Proof memory proof,) = helpers.createProof(commitment);
        string[] memory arg1 = new string[](3);
        arg1[0] = "hello";
        arg1[1] = "world";
        arg1[2] = "foo";
        uint256[] memory arg2 = new uint256[](2);
        arg2[0] = 1;
        arg2[1] = 123;
        uint argsCount = 3;
        uint dynamicArgsMap = 3; // 0x011
        uint dynamicLength = abi.encode(arg1, arg2).length - 2 * 0x20;
        Proof2 memory proof2 = Proof2(argsCount, dynamicLength, dynamicArgsMap, proof.seal, proof.commitment);
        bytes memory call = abi.encodeWithSelector(Verifier2.formatCalldata.selector, proof2, arg1, arg2, 1234, false, "DUPA", 0x1234);
        (, bytes memory result) = address(verifier).staticcall(call);
        result = abi.decode(result, (bytes));
        bytes memory expected = bytes.concat(abi.encode(proof.commitment), abi.encode(arg1, arg2, 1234));
        assertEq(result, expected);
    }
}