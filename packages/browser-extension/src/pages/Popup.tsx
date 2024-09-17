import React from "react";
import * as tlsn from "tlsn-js"

console.log("tlsn", tlsn)
export default function Popup() {
  return (
    <>
      <img src="/vlayer_logo.svg" />
      <h1>Vlayer extension</h1>
      <button onClick={() => {
          prove('https://rickandmortyapi.com/api/character/2', {
            method: 'GET',
            websocketProxyUrl: 'wss://notary.pse.dev/proxy?token=rickandmortyapi.com',
            notaryUrl: 'https://notary.pse.dev/v0.1.0-alpha.5/', 
          }
          )}}>
          prove swapi
        </button>
    </>
  );
}
