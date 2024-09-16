import React, { useEffect } from "react";
import { Button, Theme, Grid } from "@radix-ui/themes";
import browser from "webextension-polyfill";
import { useLocalStorage } from "@vlayer/extension-hooks"

import { prove } from "tlsn-js"
export default function SidePanel() {


  // cookies 
  const [authToken, setAuthToken] = useLocalStorage("authToken", "");
  const [ct0, setCt0] = useLocalStorage("ct0", "");

  // headers
  const [xCsrftoken, setXcsrftoken] = useLocalStorage("xCsrftoken", "");
  const [authorization, setAuthorization] = useLocalStorage("authorization", "");

  const [hasDataToProve, setHasDataToProve] = useLocalStorage("hasDataToProve", false);
  useEffect(() => {
    setHasDataToProve(authToken && ct0 && xCsrftoken && authorization ? true : false);
  }, [authToken, ct0, xCsrftoken, authorization]);

  useEffect(() => {
    console.log("SidePanel mounted");

    browser.webRequest.onResponseStarted.addListener(
      async (details) => {
        if (details.url.includes("api.x.com/1.1/account/settings.json")) {
          const cookies = await browser.cookies.getAll({ url: details.url });
          cookies.forEach((cookie) => {
            if (cookie.name === "auth_token") {
              setAuthToken(cookie.value);
            }
            if (cookie.name === "ct0" && cookie.value) {
              setCt0(cookie.value);
            }
          });
        }
      },
      { urls: ["<all_urls>"] },
    );

    browser.webRequest.onBeforeSendHeaders.addListener(
      (details) => {
        if (details.url.includes("api.x.com/1.1/account/settings.json")) {
          details.requestHeaders?.forEach((header) => {
            if (header.name === "authorization" && header.value) {
              setAuthorization(header.value);
            }
            if (header.name === "x-csrf-token" && header.value) {
              setXcsrftoken(header.value);
            }
          });
        }
      },
      { urls: ["<all_urls>"] },
      ["requestHeaders"]
    );
  });


  return (
    <Theme accentColor="violet">
      <img src="/vlayer_logo.svg" />
      <h1>Vlayer extension</h1>
      <Grid columns="2" gapY="4" top="8">
        <Grid columns="1" gapY="4" top="8">
          <Button variant="soft" onClick={() => {
            browser.tabs.create({ url: 'https://x.com' });
          }}> Go to x.com</Button>
          <Button disabled={hasDataToProve ? false : true} variant="soft" onClick={async () => {
            console.log("Making proof");
            try { 
              const x = await prove('https://api.x.com/1.1/account/settings.json',
                {
                  method: 'GET',
                  notaryUrl: import.meta.env.VITE_NOTARY_URL,
                  websocketProxyUrl : import.meta.env.VITE_WEBSOCKET_PROXY_URL + "?token=api.x.com",
                  headers: {
                    'x-twitter-client-language': 'en',
                    'x-csrf-token': xCsrftoken,
                    Host: 'api.x.com',
                    authorization: authorization,
                    Cookie: `lang=en; auth_token=${authToken}; ct0=${ct0}`,
                    'Accept-Encoding': 'identity',
                    Connection: 'close',
                  },
                  secretHeaders: [
                    `x-csrf-token: ${xCsrftoken}`,
                    `cookie: lang=en; auth_token=${authToken}; ct0=${ct0}`,
                    `authorization: ${authorization}`,
                  ],
                
              })              
              console.log("Proof made",x);
            } catch (e) {
              console.error("errorwhile making proof", e);
            }

          }}> Make proof</Button>

          <Button variant="soft" onClick={async () => {
             const proof = await prove('https://swapi.dev/api/people/1', {
              method: 'GET',
              headers: {
                Connection: 'close',
                Accept: 'application/json',
                'Accept-Encoding': 'identity',
              },
              body: '',
              maxTranscriptSize: 20000,
              notaryUrl: import.meta.env.VITE_NOTARY_URL,
              websocketProxyUrl: 'ws://127.0.0.1:55688',

            });
          console.log(proof);
          }
          }> Make proof for swapi</Button>

        </Grid>

      </Grid>

    </Theme>
  );
}
