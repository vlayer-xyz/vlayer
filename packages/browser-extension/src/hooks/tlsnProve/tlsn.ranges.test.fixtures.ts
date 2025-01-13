export const fixtureAllResponseHeaders = [
  "date",
  "perf",
  "pragma",
  "server",
  "status",
  "expires",
  "content-type",
  "cache-control",
  "last-modified",
  "x-transaction",
  "content-length",
  "x-access-level",
  "x-frame-options",
  "x-transaction-id",
  "x-xss-protection",
  "content-disposition",
  "x-client-event-enabled",
  "x-content-type-options",
  "x-twitter-response-tags",
  "strict-transport-security",
  "x-response-time",
  "x-connection-hash",
  "connection",
];

export const fixtureAllRequestHeaders = [
  "x-client-transaction-id",
  "accept-encoding",
  "sec-ch-ua",
  "content-type",
  "authorization",
  "host",
  "sec-ch-ua-mobile",
  "accept",
  "x-twitter-auth-type",
  "x-twitter-client-language",
  "cookie",
  "sec-ch-ua-platform",
  "x-csrf-token",
  "connection",
  "x-twitter-active-user",
  "user-agent",
];

export const fixtureAllUrlQueries = [
  "include_ext_sharing_audiospaces_listening_data_with_followers",
  "include_mention_filter",
  "include_nsfw_user_flag",
  "include_nsfw_admin_flag",
  "include_ranked_timeline",
  "include_alt_text_compose",
  "ext",
  "include_country_code",
  "include_ext_dm_nsfw_media_filter",
];

export const fixtureTranscript = {
  recv: 'HTTP/1.1 200 OK\r\ndate: Fri, 10 Jan 2025 10:54:05 GMT\r\nperf: 7402827104\r\npragma: no-cache\r\nserver: tsa_f\r\nstatus: 200 OK\r\nexpires: Tue, 31 Mar 1981 05:00:00 GMT\r\ncontent-type: application/json;charset=utf-8\r\ncache-control: no-cache, no-store, must-revalidate, pre-check=0, post-check=0\r\nlast-modified: Fri, 10 Jan 2025 10:54:05 GMT\r\nx-transaction: f7370b3d41b0ce46\r\ncontent-length: 1434\r\nx-access-level: read-write-directmessages\r\nx-frame-options: SAMEORIGIN\r\nx-transaction-id: f7370b3d41b0ce46\r\nx-xss-protection: 0\r\ncontent-disposition: attachment; filename=json.json\r\nx-client-event-enabled: true\r\nx-content-type-options: nosniff\r\nx-twitter-response-tags: BouncerCompliant\r\nstrict-transport-security: max-age=631138519\r\nx-response-time: 113\r\nx-connection-hash: 93c6c758434ff12e65de380bd00cb50530ca772501ae61a0bef72466e5624262\r\nconnection: close\r\n\r\n{"protected":false,"screen_name":"g_p_vlayer","always_use_https":true,"use_cookie_personalization":false,"sleep_time":{"enabled":false,"end_time":null,"start_time":null},"geo_enabled":false,"language":"en","discoverable_by_email":false,"discoverable_by_mobile_phone":false,"display_sensitive_media":false,"personalized_trends":true,"allow_media_tagging":"all","allow_contributor_request":"none","allow_ads_personalization":false,"allow_logged_out_device_personalization":false,"allow_location_history_personalization":false,"allow_sharing_data_for_third_party_personalization":false,"allow_dms_from":"following","always_allow_dms_from_subscribers":null,"allow_dm_groups_from":"following","translator_type":"none","country_code":"pl","nsfw_user":false,"nsfw_admin":false,"ranked_timeline_setting":null,"ranked_timeline_eligible":null,"address_book_live_sync_enabled":false,"universal_quality_filtering_enabled":"enabled","dm_receipt_setting":"all_enabled","alt_text_compose_enabled":null,"mention_filter":"unfiltered","allow_authenticated_periscope_requests":true,"protect_password_reset":false,"require_password_login":false,"requires_login_verification":false,"ext_sharing_audiospaces_listening_data_with_followers":true,"ext":{"ssoConnections":{"r":{"ok":[{"ssoIdHash":"N2duh+nd63DR7ygWST+9ItxxOov5cwKQc21zK3NXVjY=","ssoProvider":"Google"}]},"ttl":-1}},"dm_quality_filter":"enabled","autoplay_disabled":false,"settings_metadata":{}}',
  sent: 'GET https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true HTTP/1.1\r\nx-client-transaction-id: C5DC41S4o3tq5iEJw/qhLD4m2ClX6xfdbrOl7sxpS9nQgON4n5Kx0ioyf5E7ZkM3LciTOgjf2ewi/sNI6ppyegtELhSuCA\r\naccept-encoding: identity\r\nsec-ch-ua: "Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24"\r\ncontent-type: application/json\r\nauthorization: Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA\r\nhost: api.x.com\r\nsec-ch-ua-mobile: ?0\r\naccept: */*\r\nx-twitter-auth-type: OAuth2Session\r\nx-twitter-client-language: en\r\ncookie: ; night_mode=2; gt=1877662435978486236; kdt=Q3j80DP02Gpa0Tp5chMUOUKna3bcEjwj7GUbRxaT; d_prefs=MToxLGNvbnNlbnRfdmVyc2lvbjoyLHRleHRfdmVyc2lvbjoxMDAw; guest_id_ads=v1%3A173650458436725185; guest_id_marketing=v1%3A173650458436725185; personalization_id="v1_/QIcIitQ4iUSWBbX/9ShpA=="; dnt=1; auth_token=eada206578cdcc72bbb1864bc3d61ff1ab7fc44d; guest_id=v1%3A173650635437424660; ct0=cb46c0fa87a237851e1338d310c6eb921b268ce594ef71c7224b2c1aecaee2f0a1a918c218d79b2b3c4abed9b2ff75de53cd0a33d816270ae037ff9cf3e436e9f2b6587bc6e26775e1b7407ea29cba51; att=1-vXaVelR2PMWs1Gqcr7Q6mh9G5WQmQoimk7l7U0bO; twid=u%3D1835550263693766656\r\nsec-ch-ua-platform: "macOS"\r\nx-csrf-token: cb46c0fa87a237851e1338d310c6eb921b268ce594ef71c7224b2c1aecaee2f0a1a918c218d79b2b3c4abed9b2ff75de53cd0a33d816270ae037ff9cf3e436e9f2b6587bc6e26775e1b7407ea29cba51\r\nconnection: close\r\nx-twitter-active-user: yes\r\nuser-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36\r\n\r\n',
  ranges: {
    recv: {
      all: {
        start: 0,
        end: 2283,
      },
      info: {
        start: 0,
        end: 15,
      },
      headers: {
        date: {
          start: 17,
          end: 52,
        },
        perf: {
          start: 54,
          end: 70,
        },
        pragma: {
          start: 72,
          end: 88,
        },
        server: {
          start: 90,
          end: 103,
        },
        status: {
          start: 105,
          end: 119,
        },
        expires: {
          start: 121,
          end: 159,
        },
        "content-type": {
          start: 161,
          end: 205,
        },
        "cache-control": {
          start: 207,
          end: 284,
        },
        "last-modified": {
          start: 286,
          end: 330,
        },
        "x-transaction": {
          start: 332,
          end: 363,
        },
        "content-length": {
          start: 365,
          end: 385,
        },
        "x-access-level": {
          start: 387,
          end: 428,
        },
        "x-frame-options": {
          start: 430,
          end: 457,
        },
        "x-transaction-id": {
          start: 459,
          end: 493,
        },
        "x-xss-protection": {
          start: 495,
          end: 514,
        },
        "content-disposition": {
          start: 516,
          end: 567,
        },
        "x-client-event-enabled": {
          start: 569,
          end: 597,
        },
        "x-content-type-options": {
          start: 599,
          end: 630,
        },
        "x-twitter-response-tags": {
          start: 632,
          end: 673,
        },
        "strict-transport-security": {
          start: 675,
          end: 719,
        },
        "x-response-time": {
          start: 721,
          end: 741,
        },
        "x-connection-hash": {
          start: 743,
          end: 826,
        },
        connection: {
          start: 828,
          end: 845,
        },
      },
      lineBreaks: [
        {
          start: 15,
          end: 16,
        },
        {
          start: 16,
          end: 17,
        },
        {
          start: 52,
          end: 53,
        },
        {
          start: 53,
          end: 54,
        },
        {
          start: 70,
          end: 71,
        },
        {
          start: 71,
          end: 72,
        },
        {
          start: 88,
          end: 89,
        },
        {
          start: 89,
          end: 90,
        },
        {
          start: 103,
          end: 104,
        },
        {
          start: 104,
          end: 105,
        },
        {
          start: 119,
          end: 120,
        },
        {
          start: 120,
          end: 121,
        },
        {
          start: 159,
          end: 160,
        },
        {
          start: 160,
          end: 161,
        },
        {
          start: 205,
          end: 206,
        },
        {
          start: 206,
          end: 207,
        },
        {
          start: 284,
          end: 285,
        },
        {
          start: 285,
          end: 286,
        },
        {
          start: 330,
          end: 331,
        },
        {
          start: 331,
          end: 332,
        },
        {
          start: 363,
          end: 364,
        },
        {
          start: 364,
          end: 365,
        },
        {
          start: 385,
          end: 386,
        },
        {
          start: 386,
          end: 387,
        },
        {
          start: 428,
          end: 429,
        },
        {
          start: 429,
          end: 430,
        },
        {
          start: 457,
          end: 458,
        },
        {
          start: 458,
          end: 459,
        },
        {
          start: 493,
          end: 494,
        },
        {
          start: 494,
          end: 495,
        },
        {
          start: 514,
          end: 515,
        },
        {
          start: 515,
          end: 516,
        },
        {
          start: 567,
          end: 568,
        },
        {
          start: 568,
          end: 569,
        },
        {
          start: 597,
          end: 598,
        },
        {
          start: 598,
          end: 599,
        },
        {
          start: 630,
          end: 631,
        },
        {
          start: 631,
          end: 632,
        },
        {
          start: 673,
          end: 674,
        },
        {
          start: 674,
          end: 675,
        },
        {
          start: 719,
          end: 720,
        },
        {
          start: 720,
          end: 721,
        },
        {
          start: 741,
          end: 742,
        },
        {
          start: 742,
          end: 743,
        },
        {
          start: 826,
          end: 827,
        },
        {
          start: 827,
          end: 828,
        },
        {
          start: 845,
          end: 846,
        },
        {
          start: 846,
          end: 847,
        },
        {
          start: 847,
          end: 848,
        },
        {
          start: 848,
          end: 849,
        },
      ],
      body: {
        start: 849,
        end: 2282,
      },
    },
    sent: {
      all: {
        start: 0,
        end: 1868,
      },
      info: {
        start: 0,
        end: 350,
      },
      headers: {
        "x-client-transaction-id": {
          start: 352,
          end: 471,
        },
        "accept-encoding": {
          start: 473,
          end: 498,
        },
        "sec-ch-ua": {
          start: 500,
          end: 576,
        },
        "content-type": {
          start: 578,
          end: 608,
        },
        authorization: {
          start: 610,
          end: 736,
        },
        host: {
          start: 738,
          end: 753,
        },
        "sec-ch-ua-mobile": {
          start: 755,
          end: 775,
        },
        accept: {
          start: 777,
          end: 788,
        },
        "x-twitter-auth-type": {
          start: 790,
          end: 824,
        },
        "x-twitter-client-language": {
          start: 826,
          end: 855,
        },
        cookie: {
          start: 857,
          end: 1481,
        },
        "sec-ch-ua-platform": {
          start: 1483,
          end: 1510,
        },
        "x-csrf-token": {
          start: 1512,
          end: 1686,
        },
        connection: {
          start: 1688,
          end: 1705,
        },
        "x-twitter-active-user": {
          start: 1707,
          end: 1733,
        },
        "user-agent": {
          start: 1735,
          end: 1864,
        },
      },
      lineBreaks: [
        {
          start: 350,
          end: 351,
        },
        {
          start: 351,
          end: 352,
        },
        {
          start: 471,
          end: 472,
        },
        {
          start: 472,
          end: 473,
        },
        {
          start: 498,
          end: 499,
        },
        {
          start: 499,
          end: 500,
        },
        {
          start: 576,
          end: 577,
        },
        {
          start: 577,
          end: 578,
        },
        {
          start: 608,
          end: 609,
        },
        {
          start: 609,
          end: 610,
        },
        {
          start: 736,
          end: 737,
        },
        {
          start: 737,
          end: 738,
        },
        {
          start: 753,
          end: 754,
        },
        {
          start: 754,
          end: 755,
        },
        {
          start: 775,
          end: 776,
        },
        {
          start: 776,
          end: 777,
        },
        {
          start: 788,
          end: 789,
        },
        {
          start: 789,
          end: 790,
        },
        {
          start: 824,
          end: 825,
        },
        {
          start: 825,
          end: 826,
        },
        {
          start: 855,
          end: 856,
        },
        {
          start: 856,
          end: 857,
        },
        {
          start: 1481,
          end: 1482,
        },
        {
          start: 1482,
          end: 1483,
        },
        {
          start: 1510,
          end: 1511,
        },
        {
          start: 1511,
          end: 1512,
        },
        {
          start: 1686,
          end: 1687,
        },
        {
          start: 1687,
          end: 1688,
        },
        {
          start: 1705,
          end: 1706,
        },
        {
          start: 1706,
          end: 1707,
        },
        {
          start: 1733,
          end: 1734,
        },
        {
          start: 1734,
          end: 1735,
        },
        {
          start: 1864,
          end: 1865,
        },
        {
          start: 1865,
          end: 1866,
        },
        {
          start: 1866,
          end: 1867,
        },
        {
          start: 1867,
          end: 1868,
        },
      ],
    },
  },
};
