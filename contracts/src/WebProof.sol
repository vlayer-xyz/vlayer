// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct WebProof {
    string webProofJson;
}

library WebProofLib {

    address private constant VERIFY_AND_PARSE_PRECOMPILE = address(0x100);

    function verify(WebProof memory) internal pure returns (bool) {
        return true;
    }

    function url(
        WebProof memory webProof
    ) internal view returns (string memory) {
        (bool success, bytes memory returnData) = VERIFY_AND_PARSE_PRECOMPILE.staticcall(bytes(webProof.webProofJson));

        require(success, "verify_and_parse precompile call failed");

        return string(returnData);
    }

    function body(WebProof memory) internal pure returns (string memory) {
        return "";
    }

    function serverName(WebProof memory) internal pure returns (string memory) {
        return "";
    }
}
