// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "openzeppelin/contracts/utils/Strings.sol";

import {Prover} from "vlayer/Prover.sol";
import {WebProof, WebProofLib} from "vlayer/WebProof.sol";

interface IExample {
    function exampleFunction() external returns (uint256);
}

contract WebProofProver is Prover {
    using Strings for string;
    using WebProofLib for WebProof;

    constructor() {}

    function main(WebProof calldata webProof) public view returns (bool) {
        bytes calldata webProofJson = bytes(webProof.web_proof_json);
        require(webProofJson[0] == "{", "Incorrect web proof");
        require(
            webProofJson[webProofJson.length - 1] == "}",
            "Incorrect web proof"
        );

        require(
            webProof.url().equal(
                "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true"
            ),
            "Incorrect URL"
        );

        return true;
    }
}
