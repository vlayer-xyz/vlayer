// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {Web, WebProof, WebProofLib} from "vlayer-0.1.0/WebProof.sol";
import {JsonParserLib} from "vlayer-0.1.0/JsonParser.sol";

contract WebProofProver is Prover {
    using Strings for string;
    using WebProofLib for WebProof;

    string dataUrl = "https://api.x.com/1.1/account/settings.json";

    function main(WebProof calldata webProof, address account)
        public
        view
        returns (Proof memory, string memory, address)
    {
        Web memory web = webProof.verify(dataUrl);

        string memory screenName = JsonParserLib.jsonGetString(web, "screen_name");

        return (proof(), screenName, account);
    }
}
