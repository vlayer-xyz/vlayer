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
        FakeProofVerifier fakeProofVerifier = FakeProofVerifier(address(0x66aAF85f8F39dc16Da4A93CDC315B0Fa5013BcEC));
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0x038C52e00b29E6207a9CcD9bB7dbd216879B6446));
        ProofVerifierRouter proofVerifierRouter =
            ProofVerifierRouter(address(0x871F89D5871AdEE4C6A6E3F918825692b30FeBbD));

        return (fakeProofVerifier, groth16ProofVerifier, proofVerifierRouter);
    }
}
