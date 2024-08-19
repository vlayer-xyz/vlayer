// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct WebProof {
    string web_proof_json;
}

library WebProofLib {
    function verify(WebProof memory webProof) internal pure returns (bool) {
        return true;
    }

    function url(
        WebProof memory webProof
    ) internal pure returns (string memory) {
        return "api.x.com";
    }

    function body(
        WebProof memory webProof
    ) internal pure returns (string memory) {
        return "";
    }
}
