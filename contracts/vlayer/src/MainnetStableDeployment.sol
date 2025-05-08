// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";

library MainnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0x565dcA92902EA0CA597B5e62dF0b47886b6b7d4D));
    }

    function verifiers() internal pure returns (Groth16ProofVerifier) {
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0x963576398e80AC566e32b42E19C86A6580C3E7A1));

        return (groth16ProofVerifier);
    }
}
