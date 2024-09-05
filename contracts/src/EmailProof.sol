// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct EmailProof {
    string mimeEmail;
}

library EmailProofLib {
    function verify(EmailProof memory emailProof) internal pure returns (string memory) {
        return emailProof.mimeEmail;
    }
}