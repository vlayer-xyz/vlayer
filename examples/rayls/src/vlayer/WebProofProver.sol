// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer-0.1.0/WebProof.sol";
import {URLPatternLib} from "vlayer-0.1.0/URLPattern.sol";
import {Precompiles} from "vlayer-0.1.0/PrecompilesAddresses.sol";
import {Address} from "@openzeppelin-contracts-5.0.1/utils/Address.sol";

contract WebProofProver is Prover {
    using Strings for string;
    using WebProofLib for WebProof;
    using WebLib for Web;
    using URLPatternLib for string;


    string constant DATA_URL = "https://demo.tink.com/api/report?*";

    function main(WebProof calldata webProof)
        public
        view
        returns (uint256 created)
    {
        
        (bool success, bytes memory returnData) = Precompiles.VERIFY_AND_PARSE.staticcall(bytes(webProof.webProofJson));

        Address.verifyCallResult(success, returnData);

        string[4] memory data = abi.decode(returnData, (string[4]));
        require(data[0].test(DATA_URL), "Invalid data");

        Web memory web = Web(data[2], data[3]);
        string memory created = web.jsonGetString("created");

        return created;
    }
}
