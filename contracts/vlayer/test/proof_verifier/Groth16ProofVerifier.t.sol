// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test} from "forge-std-1.9.4/src/Test.sol";
import {ControlID, RiscZeroGroth16Verifier} from "risc0-ethereum-2.0.0/src/groth16/RiscZeroGroth16Verifier.sol";

import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";
import {ImageID} from "../../src/ImageID.sol";
import {ProofMode} from "../../src/Seal.sol";

import {Groth16VerifierSelector} from "../helpers/Groth16VerifierSelector.sol";
import {TestDeployer} from "../helpers/TestDeployer.sol";

contract Groth16ProofVerifier_Tests is Test {
    TestDeployer testDeployer = new TestDeployer();
    Groth16ProofVerifier immutable verifier;

    constructor() {
        verifier = testDeployer.groth16ProofVerifier();
    }

    function test_usesGroth16ProofMode() public view {
        assert(verifier.PROOF_MODE() == ProofMode.GROTH16);
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
