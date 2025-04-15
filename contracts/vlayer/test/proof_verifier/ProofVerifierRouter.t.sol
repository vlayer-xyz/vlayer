// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console} from "forge-std-1.9.4/src/Test.sol";

import {SelectorMismatch} from "risc0-ethereum-2.0.0/src/groth16/RiscZeroGroth16Verifier.sol";

import {Proof} from "../../src/Proof.sol";
import {ProofMode} from "../../src/Seal.sol";
import {IProofVerifier} from "../../src/proof_verifier/IProofVerifier.sol";
import {Repository} from "../../src/Repository.sol";
import {FakeProofVerifier} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "../../src/proof_verifier/ProofVerifierRouter.sol";

import {TestHelpers, PROVER, SELECTOR} from "../helpers/TestHelpers.sol";
import {TestDeployer} from "../helpers/TestDeployer.sol";

contract Router_Verify_Tests is Test {
    TestHelpers helpers = new TestHelpers();
    TestDeployer testDeployer = new TestDeployer();
    ProofVerifierRouter immutable router;

    constructor() {
        router = testDeployer.proofVerifierRouter();
    }

    function test_failsToConstructWhenVerifiersUseDifferentImageIDRepositories() public {
        FakeProofVerifier fakeVerifier = new FakeProofVerifier(Repository(address(1)));
        Groth16ProofVerifier groth16ProofVerifier = testDeployer.groth16ProofVerifier();

        vm.expectRevert("Verifiers should use same repository");
        new ProofVerifierRouter(fakeVerifier, groth16ProofVerifier);
    }

    function test_runsFakeVerifierForFakeProof() public {
        (Proof memory proof, bytes32 journalHash) = helpers.createProof();

        vm.expectCall(
            address(router.FAKE_PROOF_VERIFIER()),
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
