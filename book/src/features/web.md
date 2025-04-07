# Web 
<div class="feature-card feature-in-dev">
  <div class="title">
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M8.57499 3.21665L1.51665 15C1.37113 15.252 1.29413 15.5377 1.29331 15.8288C1.2925 16.1198 1.3679 16.4059 1.51201 16.6588C1.65612 16.9116 1.86392 17.1223 2.11474 17.2699C2.36556 17.4174 2.65065 17.4968 2.94165 17.5H17.0583C17.3493 17.4968 17.6344 17.4174 17.8852 17.2699C18.136 17.1223 18.3439 16.9116 18.488 16.6588C18.6321 16.4059 18.7075 16.1198 18.7067 15.8288C18.7058 15.5377 18.6288 15.252 18.4833 15L11.425 3.21665C11.2764 2.97174 11.0673 2.76925 10.8176 2.62872C10.568 2.48819 10.2864 2.41437 9.99999 2.41437C9.71354 2.41437 9.43193 2.48819 9.18232 2.62872C8.93272 2.76925 8.72355 2.97174 8.57499 3.21665V3.21665Z" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 7.5V10.8333" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 14.1667H10.0083" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
    Actively in Development
  </div>
  <p>Our team is currently working on this feature. If you experience any bugs, please let us know <a href="https://discord.gg/JS6whdessP" target="_blank">on our Discord</a>. We appreciate your patience. </p>
</div>

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

> 💡 **Try it Now**
> 
> To run the above example on your computer, type the following command in your terminal:
> 
> ```bash
> vlayer init --template simple-web-proof
> ```
> 
> This command will download all the necessary artifacts to your project.  
> The next steps are explained in [Running example](../getting-started/first-steps.md#running-examples-locally)

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

### Redaction

The TLSN protocol allows for redacting (hiding) parts of the HTTPS transcript from `Prover`, i.e. not including certain sensitive parts (e.g. cookies, authorization headers, API tokens) of the transcript in the generated Web Proof, while still being able to cryptographically prove that the rest of the transcript (the parts which are revealed) is valid.

vlayer allows for the following parts of the HTTPS transcript to be redacted:
* HTTP request:
  * URL query param values.
  * header values.
* HTTP response:
  * header values.
  * string values in JSON body.

Each value must be redacted fully or not at all. No other part of HTTP request or response can be redacted. The Solidity method `webProof.verify()` validates that these conditions are met. This way we ensure that the structure of the transcript cannot be altered by a malicious client. After redacting JSON string value for a given `"key"`, `web.jsonGetString("key")` returns a string with each byte replaced by `*` character.

In order to learn how to enable and configure redaction using vlayer SDK, see [Redaction](../javascript/web-proofs.md#redaction) section in our Javascript documentation.

### Trust Assumptions

It is important to understand that the *Notary* is a trusted party in the above setup. Since the *Notary* certifies the data, a malicious *Notary* could collude with a malicious client to create fake proofs that would still be successfully verified by `Prover`. Currently vlayer runs it's own *Notary* server, which means that vlayer needs to be trusted to certify HTTPS sessions.

 Currently vlayer also needs to be trusted when passing additional data (data other than the Web Proof itself) to `Prover` smart contract, e.g. `account` in the example above. The Web Proof could be hijacked before running `Prover` and additional data, different from the original, could be passed to `Prover`, e.g. an attacker could pass their own address as `account` in our `WebProofProver` example. Before going to production this will be addressed by making the setup trustless through an association of the additional data with a particular Web Proof in a way that's impossible to forge.

vlayer will publish a roadmap outlining how it will achieve a high level of security when using the *Notary* service.

## Quickstart guide for web proofs with custom data source
This guide walks you through setting up a development environment, configuring a custom data source, and generating/verifying Web Proofs using vlayer.

### Setting up dev environment

Initialize new project using `simple-web-proof` template:
```sh
vlayer init my-web-proof --template simple-web-proof 
```

Enter newly created project: 
```sh
cd my-web-proof
```

Build contracts: 
```sh
forge build
```
This step must be repeated whenever you modify the prover or verifier contract code. 

Navigate to the `/vlayer` directory and install JavaScript dependencies: 
```sh
cd vlayer 
bun install
```

Start local devnet: 
```sh
bun run devnet:up 
```

In separate terminal run example web application: 
```sh 
bun run web:dev 
```

### Configuring data source
Once the application is running, you can customize it for your chosen data source.
We recommend first obtaining a web proof in your browser.

To configure new data source open `vlayer/src/hooks/useTwitterAccountProof.ts` and take a look into `steps[]` attribute in `webProofConfig` object. 
Example logic is wrapped within React Hook, but you can achieve same thing in vanilla JS. 

Now you can setup new data source: 
```javascript
// specify starting page where extension redirects your user
startPage("https://x.com/", "Go to start page"), 
// in case of authentication/redirect go to specific page
expectUrl("https://x.com/home", "Log in"), 
// Specify which HTTP endpoint made by the browser from the visited website contains the data to be proven.
// This is typically an asynchronous request — you can inspect it using the Network tab in your browser’s developer tools.
notarize(
    "https://api.x.com/1.1/account/settings.json",
    "GET",
    "Generate Proof of Twitter profile",
    [
    {
        request: {
        // redact all the headers
        headers_except: [],
        },
    },
    {
        response: {
        // response from api.x.com sometimes comes with Transfer-Encoding: Chunked
        // which needs to be recognised by Prover and cannot be redacted
        headers_except: ["Transfer-Encoding"],
        },
    },
    ],
),
```

If you picked any other data source than `api.x.com` you would need to regenerate your local API token that sits in `vlayer/.env.testnet`. 

To configure `https://api.example.com` use following command: 
```sh
vlayer jwt encode -p ./fixtures/jwt-authority.key --subject deadbeef --host "api.example.com" --post 443
```

Before further development, generated token should be placed in `.env` file as `VLAYER_API_TOKEN`. Make sure that local web server got reloaded after this change. 

### Obtaining Web Proof

Now you can navigate through example app in your browser (by default running on `http://localhost:5137`) and check if [Chrome browser extension](https://chromewebstore.google.com/detail/vlayer/jbchhcgphfokabmfacnkafoeeeppjmpl) correctly redirects your user to data source. 

Finding the correct data source HTTP endpoint can be tricky. We recommend using the Network tab in your browser’s Developer Console. To narrow down the results, filter requests by Fetch/XHR, which helps isolate relevant API calls from the rest.  

A correctly generated web proof is stored in your browser's `localStorage` under the `webProof` key. To inspect `localStorage`: 
- Open Developer Console (F12 or right-click > Inspect).
- Navigate to the "Application" tab.
- In the sidebar, find "Local Storage" and select your site's domain.
- Look for the key `webProof`.

### Generating ZK proof in the prover 

After obtaining the Web Proof via the browser extension, it must be sent to the vlayer prover contract.
That is performed by `callProver()` function in `vlayer/src/components/organisms/ProveStep/Container.tsx`. 
Through vlayer sdk proof is injected into prover contract: `src/vlayer/WebProofProver.sol`. Make sure that proper URL is checked there:
```solidity
Web memory web = webProof.verify("https://api.x.com/1.1/account/settings.json");
```

Do not forget about building contracts and deploying them after any change in their code:
```sh
forge build 
cd vlayer 
bun run deploy:dev 
```

### Verifying on-chain 
Once ZK proof is returned from prover it can be used for on-chain verification. Proof along with public inputs has to passed to `WebProofVerifier.sol` using write call: 
```javascript
const writeContractArgs: Parameters<typeof writeContract>[0] = {
    address: import.meta.env.VITE_VERIFIER_ADDRESS as `0x${string}`, // Verifier contract address
    abi: webProofProofVerifier.abi, // ABI for the verifier contract
    functionName: "verify", // Function to call for verification
    args: proofData, // ZK proof data to verify (proof + public input)
};
```

Whole logic for this step is available in `handleMint()` in `vlayer/src/components/organisms/MintStep/Container.tsx` file.

### **Common Issues / FAQ**  

#### **Are there any limitations on the data that can be verified?**  
Currently, we only support JSON payloads. The maximum payload size per request and response is 10 KB. 

#### **Can I prove GraphQL responses?**  
No, this is not supported at the moment.  

#### **How can I debug extension errors?**  
1. Right-click on the extension sidebar window.  
2. Select **Inspect**.  
3. Go to the **Console** tab.  

> **Note:** The extension console is separate from your webapp console.  

#### **Can I make assertions about JSON attributes?**  
Yes, assertions must be implemented in the Prover code. You can find more details [here](/features/json-and-regex.html).  
