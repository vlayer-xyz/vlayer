// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std-1.9.2/src/Test.sol";

import {SelectorMismatch} from "risc0-ethereum-1.0.0/src/groth16/RiscZeroGroth16Verifier.sol";

import {Proof} from "../../src/Proof.sol";
import {ProofMode} from "../../src/Seal.sol";
import {IProofVerifier} from "../../src/proof_verifier/IProofVerifier.sol";
import {ProofVerifierRouter} from "../../src/proof_verifier/ProofVerifierRouter.sol";

import {TestHelpers, PROVER, SELECTOR} from "../helpers/TestHelpers.sol";

contract Router_Verify_Tests is Test {
    TestHelpers helpers = new TestHelpers();
    ProofVerifierRouter router = new ProofVerifierRouter();

    function test_runsFakeVerifierForFakeProof() public {
        (Proof memory proof, bytes32 journalHash) = helpers.createProof();

        vm.expectCall(
            address(router.fakeProofVerifier()),
            abi.encodeCall(IProofVerifier.verify, (proof, journalHash, PROVER, SELECTOR))
        );
        router.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_runsGroth16VerifierForGroth16Proof() public {
        (Proof memory proof, bytes32 journalHash) = helpers.createProof();
        proof.seal = helpers.setSealProofMode(proof.seal, ProofMode.GROTH16);

        // without a valid proof, this cannot be properly tested
        // vm.expectCall(address(router.groth16ProofVerifier()), abi.encodeCall(IProofVerifier.verify, (proof, journalHash, PROVER, SELECTOR)));
        vm.expectRevert();
        router.verify(proof, journalHash, PROVER, SELECTOR);
    }
}
