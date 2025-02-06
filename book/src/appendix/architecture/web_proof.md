The following diagram depicts the high-level architecture of how the [Web Proof](../../features/web.md) feature works:

![Architecture diagram](/images/architecture/web-proof.png)

Arrows on the diagram depict data flow between the actors (rectangles).

Generating and ZK-proving a Web Proof consits of the following steps:
1. vlayer SDK (used in a webapp) requests a Web Proof from vlayer browser extension.
2. The extension makes an HTTPS (HTTP over TLS) request to a Server through a WebSocket proxy, while conducting MPC-TLS session with the Notary, generating a Web Proof of the HTTPS request to the Server.
3. The Web Proof is then sent back to vlayer SDK.
4. vlayer SDK makes a `v_call` to vlayer Prover server, including the Web Proof as calldata to Prover Smart Contract.
5. Prover Smart Contract calls `web_proof.verify()` custom precompile, which validates the Web Proof, parses the HTTP transcript and returns it to Prover Smart Contract.
6. Prover Smart Contract then calls `json.get_string()` custom precompile, which parses JSON response body from the HTTP transcript and returns value for the specified key.
7. When Prover Smart Contract execution successfully finishes, vlayer Prover returns ZK proof of the Contract execution back to the SDK. The ZK proof can then be verified on-chain.