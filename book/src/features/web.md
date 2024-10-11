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
Let’s say we want to prove ownership of a specific Twitter/X handle.

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

    function main(WebProof calldata webProof, address twitterUserAddress)
        public
        view
        returns (Proof memory, string memory, address)
    {
        Web memory web = webProof.verify(dataUrl);

        string memory screenName = web.jsonGetString("screen_name");

        return (proof(), screenName, twitterUserAddress);
    }
}
```

What happens in the above code?  

1. **Setup the `Prover` contract**:
   - `WebProofProver` inherits from the `Prover` contract, enabling off-chain proving of web data.
   - The `main` function receives a `WebProof`, which contains a signed transcript of an HTTPS session (see the [Security Considerations](#security-considerations) section for details about the TLS *Notary*).

2. **Verify the Web Proof**:
   - The call to `webProof.verify(dataUrl)` does the following:
     - Verifies the HTTPS transcript.
     - Verifies the *Notary*'s signature on the transcript.
     - Ensures the *Notary* is trusted (via their signing key).
     - Confirms the data comes from the expected domain (`api.x.com` in this case).
     - Check whether the HTTPS data comes from the expected `dataUrl`.
     - Ensures that the server's SSL certificate and its chain of authority are verified.
     - Retrieves the plain text transcript for further processing.

3. **Extract the relevant data**:
   - `web.jsonGetString("screen_name")` extracts the `screen_name` from the JSON response.

4. **Return the results**:
   - If everything checks out, the function returns the `proof` placeholder, `screenName`, and the `twitterUserAddress`.

If there are no errors and the proof is valid, the data is ready for on-chain verification.

> 💡 **Try it Now**
> 
> To run the above example on your computer, type the following command in your terminal:
> 
> ```bash
> vlayer init --template web-proof
> ```
> 
> This command will download all the necessary artifacts to your project.  
> The next steps are explained in [Running example](../getting-started/first-steps.md#running-examples-locally)

## Example Verifier
The contract below mints a unique NFT for the Twitter/X handle owner’s wallet address.

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

    function verify(Proof calldata, string memory username, address twitterUserAddress)
        public
        onlyVerified(prover, WebProofProver.main.selector)
    {
        require(twitterUserAddress == msg.sender, "Wrong caller");

        uint256 tokenId = uint256(keccak256(abi.encodePacked(username)));
        require(_ownerOf(tokenId) == address(0), "User has already minted a TwitterNFT");

        _safeMint(msg.sender, tokenId);
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
   - The `msg.sender` must match the address previously associated with the handle.
   - The `tokenId` (a hash of the handle) must not already be minted.

3. **Mint the NFT**:
   - Once verified, a unique `TwitterNFT` is minted for the user.

And that's it! 

As you can see, Web Proofs can be a powerful tool for building decentralized applications by allowing trusted off-chain data to interact with smart contracts.

## Security Considerations

The Web Proof feature is based on the [TLSNotary](https://tlsnotary.org/) protocol. Web data is retrieved from an HTTP endpoint and it's integrity and authenticity during the HTTP session is verified using the Transport Layer Security (TLS) protocol (the "S" in HTTPS), which secures most modern encrypted connections on the Internet. Web Proofs ensure the integrity and authenticity of web data after the HTTPS session finishes by extending the TLS protocol. A designated server, called *Notary*, joins the HTTPS session between the client and the server and can cryptographically certify its contents.

From privacy perspective, it is important to note that the *Notary* server never has access to the plaintext transcript of the connection and therefore, *Notary* can never steal client data and pretend to be client. Furthermore, the transcript can be redacted (i.e. certain parts can be removed) by the client, making these parts of the communication not accessible by `Prover` and vlayer infrastructure running the `Prover`.

### Trust Assumptions

It is important to understand that the *Notary* is a trusted party in the above setup. Since the *Notary* certifies the data, a malicious *Notary* could collude with a malicious client to create fake proofs that would still be successfully verified by `Prover`. Currently vlayer runs it's own *Notary* server, which means that vlayer needs to be trusted to certify HTTPS sessions.

 Currently vlayer also needs to be trusted when passing additional data (data other than the Web Proof itself) to `Prover` smart contract, e.g. `twitterUserAddress` in the example above. The Web Proof could be hijacked before running `Prover` and additional data, different from the original, could be passed to `Prover`, e.g. an attacker could pass their own address as `twitterUserAddress` in our `WebProofProver` example. Before going to production this will be addressed by making the setup trustless through an association of the additional data with a particular Web Proof in a way that's impossible to forge.

vlayer will publish a roadmap outlining how it will achieve a high level of security when using the *Notary* service.
