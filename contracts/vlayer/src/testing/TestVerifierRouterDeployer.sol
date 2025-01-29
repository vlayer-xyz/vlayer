// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ProofVerifierRouter, IImageIdRepository, FakeProofVerifier, Groth16ProofVerifier} from "../proof_verifier/ProofVerifierRouter.sol";
import {Verifier} from "../Verifier.sol";

contract TestVerifierRouterDeployer {
    function swapProofVerifier(Verifier verifier) external {
        IImageIdRepository repository = verifier.verifier().imageIdRepository();
        verifier._setTestVerifier(new ProofVerifierRouter(new FakeProofVerifier(repository), new Groth16ProofVerifier(repository)));
    }
}