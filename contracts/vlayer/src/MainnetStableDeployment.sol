// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";

library MainnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0xbDf27a6f3CF309F9127d8173d0D28bF9ab35ed2b));
    }

    function verifiers() internal pure returns (Groth16ProofVerifier) {
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0x1EE8a3B907EbcdFc33f76e3C7aAe6FFD2eFA5b73));

        return (groth16ProofVerifier);
    }
}
