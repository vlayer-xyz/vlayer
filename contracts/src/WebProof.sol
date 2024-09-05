// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "@openzeppelin-contracts-5.0.2/utils/Strings.sol";

struct WebProof {
    string webProofJson;
}

library WebProofLib {
    using Strings for string;

    address private constant VERIFY_AND_PARSE_PRECOMPILE = address(0x100);

    function verify(
        WebProof memory webProof,
        string memory dataUrl
    ) internal view returns (bool) {
        (bool success, bytes memory returnData) = VERIFY_AND_PARSE_PRECOMPILE
            .staticcall(bytes(webProof.webProofJson));

        require(success, "verify_and_parse precompile call failed");

        string memory url = string(returnData);
        require(dataUrl.equal(url), "Incorrect URL");

        return true;
    }
}
