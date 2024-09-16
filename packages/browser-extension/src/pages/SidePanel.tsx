import React, { useEffect } from "react";
import { Button, Theme, Grid, Spinner} from "@radix-ui/themes";
import browser from "webextension-polyfill";
import { useLocalStorage } from "@vlayer/extension-hooks"

import { prove, verify } from "tlsn-js"
export default function SidePanel() {
  


  // cookies 
  const [authToken, setAuthToken] = useLocalStorage("authToken", "");
  const [ct0, setCt0] = useLocalStorage("ct0", "");

  // headers
  const [xCsrftoken, setXcsrftoken] = useLocalStorage("xCsrftoken", "");
  const [authorization, setAuthorization] = useLocalStorage("authorization", "");

  const [hasDataToProve, setHasDataToProve] = useLocalStorage("hasDataToProve", false);
  const [ isProoving, setIsProving] = useLocalStorage("isProving", false);
  const [isProofReady, setIsProofReady] = useLocalStorage("isProofReady", false);
  const [proof, setProof] = useLocalStorage<any>("proof", "");
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
    <Theme accentColor="violet" hasBackground={true} panelBackground="solid">
      <Grid columns="8" gapY="4" top="16" style={{marginTop : '80px'}} >
        <div style={{gridColumn : 'span 1'}}></div>
        <div style={{gridColumn: 'span 6'}}>
          <Grid columns="1" gapY="4">
          <Button variant="soft" onClick={() => {
            browser.tabs.create({ url: 'https://x.com' });
          }}> Go to x.com</Button>
          <Button disabled={hasDataToProve && !isProoving? false : true} variant="soft" onClick={async () => {
            if ( isProofReady) {
              browser.tabs.create({ url: 'http://localhost:5134' });
            } else {
              setIsProving(true);
              try { 
                const proof = await prove('https://api.x.com/1.1/account/settings.json',
                  {
                    method: 'GET',
                    notaryUrl: 'http://localhost:7047',
                    websocketProxyUrl: 'ws://localhost:55689',
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
                setProof(proof);
                setIsProofReady(true);
                setIsProving(false);
              } catch (e) {
                console.error("errorwhile making proof", e);
              }
            }
          }}> { isProoving ? <Spinner/> : isProofReady? "Back to home with proove" : "Make Proof"}</Button>
{/* 
          <Button disabled={isProofReady ? false : true} variant="soft" onClick={() => {
            browser.tabs.create({ url: 'http://localhost:5134' });
          }}> Back to home page</Button> */}

          {/* <Button variant="soft" onClick={async () => {

             const proof = await prove('https://rickandmortyapi.com/api/character/2', {
              method: 'GET',
              notaryUrl: 'http://localhost:7047',
              websocketProxyUrl: 'ws://localhost:55688',
            });
          console.log(proof);
          }
          }> Make proof for swapi</Button> */}

          
        </Grid>
        </div>

      </Grid>

    </Theme>
  );
}
