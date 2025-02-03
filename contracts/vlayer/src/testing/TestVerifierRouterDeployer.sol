// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {
    ProofVerifierRouter,
    IImageIdRepository,
    FakeProofVerifier,
    Groth16ProofVerifier,
    IProofVerifier
} from "../proof_verifier/ProofVerifierRouter.sol";

interface ITestingVerifier {
    function verifier() external view returns (IProofVerifier);
    function setTestVerifier(IProofVerifier newVerifier) external;
}

contract TestVerifierRouterDeployer {
    function swapProofVerifier(ITestingVerifier verifier) external {
        IImageIdRepository repository = verifier.verifier().imageIdRepository();
        verifier.setTestVerifier(
            new ProofVerifierRouter(new FakeProofVerifier(repository), new Groth16ProofVerifier(repository))
        );
    }
}
