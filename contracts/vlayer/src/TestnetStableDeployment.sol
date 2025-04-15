// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {FakeProofVerifier} from "./proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "./proof_verifier/ProofVerifierRouter.sol";

library TestnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0xc4E4dC291A5C4dEbe9Ff5a3372F3FdD2e42Bac86));
    }

    function verifiers() internal pure returns (FakeProofVerifier, Groth16ProofVerifier, ProofVerifierRouter) {
        FakeProofVerifier fakeProofVerifier = FakeProofVerifier(address(0xb375BdA1F2fFE447DF77Eb155544E48307235360));
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0x7E231CfC3e3B549633D5AD61C30f07Dd4d408ad3));
        ProofVerifierRouter proofVerifierRouter =
            ProofVerifierRouter(address(0x6C71ff0E0b1848D3C040Fb4908B5eFc451C1EAbC));

        return (fakeProofVerifier, groth16ProofVerifier, proofVerifierRouter);
    }
}
