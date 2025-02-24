import { MessageTranscript } from "./types";
import { EncodedString, Encoding, parseHttpMessage } from "./utils";

export const extractHeaders = (httpMessage: string) => {
  const headerSection = httpMessage.split("\r\n\r\n")[0];
  const headerLines = headerSection.split("\r\n").slice(1); // Skip first line (status/request line)
  const headers: string[] = [];

  for (const line of headerLines) {
    const headerName = line.split(":")[0].toLowerCase().trim();
    if (headerName) {
      headers.push(headerName);
    }
  }
  return headers;
};

export const extractUrlQueryParams = (httpMessage: string) => {
  const requestLine = httpMessage.split("\r\n")[0];
  const urlMatch = requestLine.match(/\?([^?\s]+)/);

  if (!urlMatch) {
    return [];
  }

  const queryString = urlMatch[1];
  const params = queryString.split("&").map((param) => {
    return param.split("=")[0];
  });

  return params;
};

export const getHeaderRange = (
  transcript: MessageTranscript,
  header: string,
) => {
  const headerStart =
    transcript.message.content.indexOf("\r\n" + header + ": ") +
    new EncodedString(`\r\n${header}: `, transcript.encoding).length;
  const headerEnd = transcript.message.content.indexOf("\r\n", headerStart);

  return {
    start: headerStart,
    end: headerEnd === -1 ? transcript.message.content.length : headerEnd,
  };
};

// recorded tlsn output
const XAPICallTranscriptTlsn = {
  recv: 'HTTP/1.1 200 OK\r\ndate: Fri, 10 Jan 2025 10:54:05 GMT\r\nperf: 7402827104\r\npragma: no-cache\r\nserver: tsa_f\r\nstatus: 200 OK\r\nexpires: Tue, 31 Mar 1981 05:00:00 GMT\r\ncontent-type: application/json;charset=utf-8\r\ncache-control: no-cache, no-store, must-revalidate, pre-check=0, post-check=0\r\nlast-modified: Fri, 10 Jan 2025 10:54:05 GMT\r\nx-transaction: f7370b3d41b0ce46\r\ncontent-length: 1434\r\nx-access-level: read-write-directmessages\r\nx-frame-options: SAMEORIGIN\r\nx-transaction-id: f7370b3d41b0ce46\r\nx-xss-protection: 0\r\ncontent-disposition: attachment; filename=json.json\r\nx-client-event-enabled: true\r\nx-content-type-options: nosniff\r\nx-twitter-response-tags: BouncerCompliant\r\nstrict-transport-security: max-age=631138519\r\nx-response-time: 113\r\nx-connection-hash: 93c6c758434ff12e65de380bd00cb50530ca772501ae61a0bef72466e5624262\r\nconnection: close\r\n\r\n{"protected":false,"screen_name":"g_p_vlayer","always_use_https":true,"use_cookie_personalization":false,"sleep_time":{"enabled":false,"end_time":null,"start_time":null},"geo_enabled":false,"language":"en","discoverable_by_email":false,"discoverable_by_mobile_phone":false,"display_sensitive_media":false,"personalized_trends":true,"allow_media_tagging":"all","allow_contributor_request":"none","allow_ads_personalization":false,"allow_logged_out_device_personalization":false,"allow_location_history_personalization":false,"allow_sharing_data_for_third_party_personalization":false,"allow_dms_from":"following","always_allow_dms_from_subscribers":null,"allow_dm_groups_from":"following","translator_type":"none","country_code":"pl","nsfw_user":false,"nsfw_admin":false,"ranked_timeline_setting":null,"ranked_timeline_eligible":null,"address_book_live_sync_enabled":false,"universal_quality_filtering_enabled":"enabled","dm_receipt_setting":"all_enabled","alt_text_compose_enabled":null,"mention_filter":"unfiltered","allow_authenticated_periscope_requests":true,"protect_password_reset":false,"require_password_login":false,"requires_login_verification":false,"ext_sharing_audiospaces_listening_data_with_followers":true,"ext":{"ssoConnections":{"r":{"ok":[{"ssoIdHash":"N2duh+nd63DR7ygWST+9ItxxOov5cwKQc21zK3NXVjY=","ssoProvider":"Google"}]},"ttl":-1}},"dm_quality_filter":"enabled","autoplay_disabled":false,"settings_metadata":{}}',
  sent: 'GET https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true HTTP/1.1\r\nx-client-transaction-id: C5DC41S4o3tq5iEJw/qhLD4m2ClX6xfdbrOl7sxpS9nQgON4n5Kx0ioyf5E7ZkM3LciTOgjf2ewi/sNI6ppyegtELhSuCA\r\naccept-encoding: identity\r\nsec-ch-ua: "Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24"\r\ncontent-type: application/json\r\nauthorization: Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA\r\nhost: api.x.com\r\nsec-ch-ua-mobile: ?0\r\naccept: */*\r\nx-twitter-auth-type: OAuth2Session\r\nx-twitter-client-language: en\r\ncookie: ; night_mode=2; gt=1877662435978486236; kdt=Q3j80DP02Gpa0Tp5chMUOUKna3bcEjwj7GUbRxaT; d_prefs=MToxLGNvbnNlbnRfdmVyc2lvbjoyLHRleHRfdmVyc2lvbjoxMDAw; guest_id_ads=v1%3A173650458436725185; guest_id_marketing=v1%3A173650458436725185; personalization_id="v1_/QIcIitQ4iUSWBbX/9ShpA=="; dnt=1; auth_token=eada206578cdcc72bbb1864bc3d61ff1ab7fc44d; guest_id=v1%3A173650635437424660; ct0=cb46c0fa87a237851e1338d310c6eb921b268ce594ef71c7224b2c1aecaee2f0a1a918c218d79b2b3c4abed9b2ff75de53cd0a33d816270ae037ff9cf3e436e9f2b6587bc6e26775e1b7407ea29cba51; att=1-vXaVelR2PMWs1Gqcr7Q6mh9G5WQmQoimk7l7U0bO; twid=u%3D1835550263693766656\r\nsec-ch-ua-platform: "macOS"\r\nx-csrf-token: cb46c0fa87a237851e1338d310c6eb921b268ce594ef71c7224b2c1aecaee2f0a1a918c218d79b2b3c4abed9b2ff75de53cd0a33d816270ae037ff9cf3e436e9f2b6587bc6e26775e1b7407ea29cba51\r\nconnection: close\r\nx-twitter-active-user: yes\r\nuser-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36\r\n\r\n',
};

export const XAPICallTranscript = {
  recv: parseHttpMessage(XAPICallTranscriptTlsn.recv),
  sent: parseHttpMessage(XAPICallTranscriptTlsn.sent, {
    enforceContentType: false,
    defaultEncoding: Encoding.UTF8,
  }),
};

const TlsnTranscriptWithDoubleHeaders = {
  recv: "HTTP/1.1 200 OK\r\ndate: Wed, 19 Feb 2025 15:55:59 GMT\r\ndate: Wed, 19 Feb 2025 15:55:59 GMT\r\ncontent-type: application/json; charset=utf-8\r\n\r\n",
  sent: "GET https://x.com/i/api\r\nhost: x.com\r\nhost: x.com\r\nhost: x.com\r\nhost: x.com\r\nhost: x.com\r\n\r\n",
};

export const TranscriptWithDoubleHeaders = {
  recv: parseHttpMessage(TlsnTranscriptWithDoubleHeaders.recv),
  sent: parseHttpMessage(TlsnTranscriptWithDoubleHeaders.sent, {
    enforceContentType: false,
    defaultEncoding: Encoding.UTF8,
  }),
};

export const allRequestHeadersRedactedRanges = extractHeaders(
  XAPICallTranscript.sent.message.content.toString(),
).map((header) => getHeaderRange(XAPICallTranscript.sent, header));

export const allResponseHeadersRedactedRanges = extractHeaders(
  XAPICallTranscript.recv.message.content.toString(),
).map((header) => getHeaderRange(XAPICallTranscript.recv, header));
