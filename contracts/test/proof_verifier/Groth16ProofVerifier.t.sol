// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";

import {ControlID, RiscZeroGroth16Verifier} from "risc0-ethereum/groth16/RiscZeroGroth16Verifier.sol";

import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";
import {ProofMode} from "../../src/Seal.sol";

contract FakeProofVerifier_Tests is Test {
    Groth16ProofVerifier verifier = new Groth16ProofVerifier();

    function test_usesGroth16ProofMode() public view {
        assert(verifier.proofMode() == ProofMode.GROTH16);
    }

    function test_usesGroth16RiscZeroVerifier() public {
        RiscZeroGroth16Verifier mockVerifier =
            new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
        assertEq(address(verifier.verifier()).codehash, address(mockVerifier).codehash);
    }
}
