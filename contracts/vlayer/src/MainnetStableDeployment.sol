// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";

library MainnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0x42fc5CdBfA5E4699C0e1e0adD0c4BC421d80482F));
    }

    function verifiers() internal pure returns (Groth16ProofVerifier) {
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0xb8Be5BdCD6387332448f551cFe7684e50d9E108C));

        return (groth16ProofVerifier);
    }
}
