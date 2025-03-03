## High level architecture

The following diagram depicts the high-level architecture of how the [Web Proof](../../features/web.md) feature works:

![Architecture diagram](/images/architecture/web-proof.png)

Arrows on the diagram depict data flow between the actors (rectangles).

Generating and ZK-proving a Web Proof consits of the following steps:
1. vlayer [SDK](../../javascript/javascript.md) (used in a webapp) requests a Web Proof from vlayer browser extension.
2. The extension opens a TLS connection to a Server (2a) through a WebSocket proxy (2b), while conducting MPC-TLS session with the Notary (2c), generating a Web Proof of an HTTPS request to the Server. The WebSocket proxy is needed to provide the extension access to low-level details of the TLS handshake, which is normally not available within the browser, while the Notary acts as a trusted third party who certifies the transcript of the HTTPS request (without actually seeing it). For details of how the TLSN protocol works, please check [TLSN documentation](https://docs.tlsnotary.org/).
3. The Web Proof is then sent back to vlayer SDK.
4. vlayer SDK makes a [`v_call`](../api.md#v_call) to vlayer [Prover server](./prover.md), including the Web Proof as calldata to [Prover Smart Contract](../../features/web.md#example-prover).
5. Prover Smart Contract calls `web_proof.verify()` custom precompile (see [Precompiles](./prover.md#precompiles)), which validates the Web Proof, parses the HTTP transcript and returns it to Prover Smart Contract.
6. Prover Smart Contract then calls `json.get_string()` custom precompile (see [Precompiles](./prover.md#precompiles)), which parses JSON response body from the HTTP transcript and returns value for the specified key.
7. When Prover Smart Contract execution successfully finishes, vlayer Prover returns ZK proof of the Contract execution back to the SDK. The ZK proof can then be verified on-chain.
