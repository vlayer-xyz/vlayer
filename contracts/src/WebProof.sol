// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "openzeppelin/contracts/utils/Strings.sol";

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

        string[3] memory data = abi.decode(returnData, (string[3]));
        string memory serverName = "api.x.com";
        string
            memory body = '{"protected":false,"screen_name":"jab68503","always_use_https":true,"use_cookie_personalization":false,"sleep_time":{"enabled":false,"end_time":null,"start_time":null},"geo_enabled":false,"language":"en","discoverable_by_email":false,"discoverable_by_mobile_phone":false,"display_sensitive_media":false,"personalized_trends":true,"allow_media_tagging":"all","allow_contributor_request":"none","allow_ads_personalization":false,"allow_logged_out_device_personalization":false,"allow_location_history_personalization":false,"allow_sharing_data_for_third_party_personalization":false,"allow_dms_from":"following","always_allow_dms_from_subscribers":null,"allow_dm_groups_from":"following","translator_type":"none","country_code":"pl","nsfw_user":false,"nsfw_admin":false,"ranked_timeline_setting":null,"ranked_timeline_eligible":null,"address_book_live_sync_enabled":false,"universal_quality_filtering_enabled":"enabled","dm_receipt_setting":"all_enabled","alt_text_compose_enabled":null,"mention_filter":"unfiltered","allow_authenticated_periscope_requests":true,"protect_password_reset":false,"require_password_login":false,"requires_login_verification":false,"ext_sharing_audiospaces_listening_data_with_followers":true,"ext":{"ssoConnections":{"r":{"ok":[{"ssoIdHash":"P4GxOpBmKVdXcOWBZkVUlIJgrojh9RBwDDAEkGXK6VQ=","ssoProvider":"Google"}]},"ttl":-1}},"dm_quality_filter":"enabled","autoplay_disabled":false,"settings_metadata":{}}';

        require(success, "verify_and_parse precompile call failed");
        require(dataUrl.equal(data[0]), "Incorrect URL");
        require(serverName.equal(data[1]), "Server name not found");
        require(body.equal(data[2]), "Body not found");

        return true;
    }
}
