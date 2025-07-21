// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";

library MainnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0x9DFcaBBDbe296E5C4bf36E4431C46daB43702c84));
    }

    function verifiers() internal pure returns (Groth16ProofVerifier) {
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0xc4B7dEd1C30ec34802c85B8345eaC15c02d646A0));

        return (groth16ProofVerifier);
    }
}
