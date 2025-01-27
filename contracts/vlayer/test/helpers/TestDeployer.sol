// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {console} from "forge-std-1.9.4/src/Test.sol";

import {ImageID} from "../../src/ImageID.sol";

import {ImageIdRepository} from "../../src/proof_verifier/ImageIdRepository.sol";
import {FakeProofVerifier} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "../../src/proof_verifier/ProofVerifierRouter.sol";

contract TestDeployer {
    ImageIdRepository public immutable repository;
    FakeProofVerifier public immutable fakeProofVerifier;
    Groth16ProofVerifier public immutable groth16ProofVerifier;
    ProofVerifierRouter public immutable proofVerifierRouter;

    constructor() {
        repository = new ImageIdRepository(address(this), address(this));
        repository.addSupport(ImageID.RISC0_CALL_GUEST_ID);

        fakeProofVerifier = new FakeProofVerifier(repository);
        groth16ProofVerifier = new Groth16ProofVerifier(repository);

        proofVerifierRouter = new ProofVerifierRouter(fakeProofVerifier, groth16ProofVerifier);
    }
}
