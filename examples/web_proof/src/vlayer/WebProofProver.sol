// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

import {Prover} from "vlayer-0.1.0/src/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer-0.1.0/src/WebProof.sol";

interface IExample {
    function exampleFunction() external returns (uint256);
}

contract WebProofProver is Prover {
    using Strings for string;
    using WebProofLib for WebProof;
    using WebLib for Web;

    string dataUrl =
        "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true";

    function main(WebProof calldata webProof) public view returns (string memory) {
        Web memory web = webProof.verify(dataUrl);

        string memory screenName = web.jsonGetString("screen_name");

        require(screenName.equal("jab68503"), "Invalid screen_name");

        return screenName;
    }
}
