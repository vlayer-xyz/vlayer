# Web 
Existing web applications including finance, social media, government, ecommerce and many other types of services contain valuable information and can be turned into great data sources.

With vlayer, you can leverage **this data** in smart contracts.

## Web Proofs
Web Proofs provide cryptographic proof of web data served by any HTTPS server, allowing developers to use this data in smart contracts. Only a small subset of the required data is published on-chain.

Web Proofs ensure that the data received has not been tampered with. Without Web Proofs, proving this on-chain is difficult, especially when aiming for an automated and trusted solution.

## Example Prover
Let's say we want to mint an NFT for a wallet address linked to a specific X/Twitter handle.

Here’s a sample Prover contract:

```solidity
import {Strings} from "@openzeppelin-contracts/utils/Strings.sol";
import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer-0.1.0/WebProof.sol";

contract WebProofProver is Prover {
    using Strings for string;
    using WebProofLib for WebProof;
    using WebLib for Web;

    string dataUrl = "https://api.x.com/1.1/account/settings.json";

    function main(WebProof calldata webProof, address account)
        public
        view
        returns (Proof memory, string memory, address)
    {
        Web memory web = webProof.verify(dataUrl);

        string memory screenName = web.jsonGetString("screen_name");

        return (proof(), screenName, account);
    }
}
```

What happens in the above code?  

1. **Setup the `Prover` contract**:
    - `WebProofProver` inherits from the `Prover` contract, enabling off-chain proving of web data.
    - The `main` function receives a `WebProof`, which contains a signed transcript of an HTTPS session (see the chapter from [JS section](../javascript/web-proofs.md) on how to obtain `WebProof`). The transcript is signed by a *Notary* (see [Security Considerations](#security-considerations) section for details about the TLS *Notary*).

2. **Verify the Web Proof**:
    
    The call to `webProof.verify(dataUrl)` does the following:
    - Verifies the HTTPS transcript.
    - Verifies the *Notary*'s signature on the transcript.
    - Ensures the *Notary* is on the list of trusted notaries (via their signing key).
    - Confirms the data comes from the expected domain (`api.x.com` in this case).
    - Check whether the HTTPS data comes from the expected `dataUrl`. `dataUrl` is a [URL Pattern](https://urlpattern.spec.whatwg.org/) against which the actual URL is checked.
    - Ensures that the server's SSL certificate and its chain of authority are verified.
    - Retrieves the plain text transcript for further processing.

3. **Extract the relevant data**:
    
    `web.jsonGetString("screen_name")` extracts the `screen_name` from the JSON response.

4. **Return the results**:

    If everything checks out, the function returns the `proof` placeholder, `screenName`, and the `account`.

If there are no errors and the proof is valid, the data is ready for on-chain verification. 

## Obtaining Web Proofs

vlayer provides two ways to obtain Web Proofs:

- **Client-side** (via browser extension)  
- **Server-side** (via the vlayer CLI)

### Client-side

The client-side method is intended for scenarios where the target data is authenticated using browser cookies (e.g., logged-in user sessions). It uses the [vlayer browser extension](https://chromewebstore.google.com/detail/vlayer/jbchhcgphfokabmfacnkafoeeeppjmpl) to capture and notarize HTTP requests directly from the user’s browsing context.

This approach is ideal for proving access to **social media activity**, **personal or banking data**, **Web2 loyalty points**, **reputation scores**, or any other cookie-protected content.

> 💡 **Try it Now**
>
> To run an example that proves ownership of an X/Twitter handle on your computer, enter the following command in your terminal:
>
> ```bash
> vlayer init --template simple-web-proof
> ```
>
> This command will download all the necessary artifacts for your project.  
> The next steps are explained in the [Running Examples](../getting-started/first-steps.md#running-examples-locally) and [Quickstart Guide](/web-proof/quickstart-guide.html).

### Server-side

The server-side method is intended for proving data retrieved from HTTP requests that are either public or authenticated via token. It’s a great fit for APIs such as AI models, fintech services, or any backend integration where browser cookie is not required.

> 💡 **Try it Now**
>
> To run an example that proves data returned by the Kraken API, enter the following command in your terminal:
>
> ```bash
> vlayer init --template kraken-web-proof
> ```
>
> This will download all necessary artifacts to your project.  
> The next steps are detailed in [Running Examples](../getting-started/first-steps.md#running-examples-locally).

## Example Verifier
The contract below verifies provided Web Proof and mints a unique NFT for the Twitter/X handle owner’s wallet address.

```solidity
import {WebProofProver} from "./WebProofProver.sol";
import {Proof} from "vlayer/Proof.sol";
import {Verifier} from "vlayer/Verifier.sol";

import {ERC721} from "@openzeppelin-contracts/token/ERC721/ERC721.sol";

contract WebProofVerifier is Verifier, ERC721 {
    address public prover;

    constructor(address _prover) ERC721("TwitterNFT", "TNFT") {
        prover = _prover;
    }

    function verify(Proof calldata, string memory username, address account)
        public
        onlyVerified(prover, WebProofProver.main.selector)
    {
        uint256 tokenId = uint256(keccak256(abi.encodePacked(username)));
        require(_ownerOf(tokenId) == address(0), "User has already minted a TwitterNFT");

        _safeMint(account, tokenId);
    }
}

```
What’s happening here?

1. **Set up the `Verifier`**:
    - The `prover` variable stores the address of the `Prover` contract that generated the proof.
    - The `WebProofProver.main.selector` gets the selector for the `WebProofProver.main()` function.
    - `WebProofVerifier` inherits from `Verifier` to access the `onlyVerified` modifier, which ensures the proof is valid.
    - `WebProofVerifier` also inherits from `ERC721` to support NFTs.

2. **Verification checks**:

    The `tokenId` (a hash of the handle) must not already be minted.

3. **Mint the NFT**:

    Once verified, a unique `TwitterNFT` is minted for the user.

And that's it! 

As you can see, Web Proofs can be a powerful tool for building decentralized applications by allowing trusted off-chain data to interact with smart contracts.

## Notary 
A *Notary* is a third-party server that participates in a two-sided Transport Layer Security (TLS) session between a client and a server. Its role is to attest that specific communication has occurred between the two parties.

## Security Considerations

The Web Proof feature is based on the [TLSNotary](https://tlsnotary.org/) protocol. Web data is retrieved from an HTTP endpoint and it's integrity and authenticity during the HTTP session is verified using the TLS protocol (the "S" in HTTPS), which secures most modern encrypted connections on the Internet. Web Proofs ensure the integrity and authenticity of web data after the HTTPS session finishes by extending the TLS protocol. *Notary*, joins the HTTPS session between the client and the server and can cryptographically certify its contents.

From privacy perspective, it is important to note that the *Notary* server never has access to the plaintext transcript of the connection and therefore, *Notary* can never steal client data and pretend to be client. Furthermore, the transcript can be redacted (i.e. certain parts can be removed) by the client, making these parts of the communication not accessible by `Prover` and vlayer infrastructure running the `Prover`.

### [Redaction](../web-proof/redaction.md)

### Trust Assumptions

It is important to understand that the *Notary* is a trusted party in the above setup. Since the *Notary* certifies the data, a malicious *Notary* could collude with a malicious client to create fake proofs that would still be successfully verified by `Prover`. Currently vlayer runs it's own *Notary* server, which means that vlayer needs to be trusted to certify HTTPS sessions.

 Currently vlayer also needs to be trusted when passing additional data (data other than the Web Proof itself) to `Prover` smart contract, e.g. `account` in the example above. The Web Proof could be hijacked before running `Prover` and additional data, different from the original, could be passed to `Prover`, e.g. an attacker could pass their own address as `account` in our `WebProofProver` example. Before going to production this will be addressed by making the setup trustless through an association of the additional data with a particular Web Proof in a way that's impossible to forge.

vlayer will publish a roadmap outlining how it will achieve a high level of security when using the *Notary* service.
