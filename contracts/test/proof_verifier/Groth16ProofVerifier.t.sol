// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std-1.9.2/src/Test.sol";
import {ControlID, RiscZeroGroth16Verifier} from "risc0-ethereum-1.0.0/src/groth16/RiscZeroGroth16Verifier.sol";

import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";
import {ImageID} from "../../src/ImageID.sol";
import {ProofMode} from "../../src/Seal.sol";

import {Groth16VerifierSelector} from "../helpers/Groth16VerifierSelector.sol";

contract Groth16ProofVerifier_Tests is Test {
    Groth16ProofVerifier public verifier = new Groth16ProofVerifier();

    function test_usesGroth16ProofMode() public view {
        assert(verifier.PROOF_MODE() == ProofMode.GROTH16);
    }

    function test_usesProperImageId() public view {
        assert(verifier.CALL_GUEST_ID() == ImageID.RISC0_CALL_GUEST_ID);
    }

    function test_usesGroth16RiscZeroVerifier() public {
        RiscZeroGroth16Verifier mockVerifier =
            new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
        assertEq(address(verifier.VERIFIER()).codehash, address(mockVerifier).codehash);
    }

    function test_verifierSelectorIsStable() public {
        RiscZeroGroth16Verifier mockVerifier =
            new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);

        assertEq(Groth16VerifierSelector.STABLE_VERIFIER_SELECTOR, mockVerifier.SELECTOR());
    }
}
