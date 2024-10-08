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
  <p>Our team is currently working on this feature. In case of any bug please retry in 1-2 weeks. We appreciate your patience. </p>
</div>
Existing web applications including finance, social media, government, ecommerce and many other types of services contain valuable information and can be turned into great data sources.

With vlayer, it is possible to leverage **this data** with smart contracts.

## Web Proofs
Web Proofs provide cryptographic proof of web data served by any HTTPS server, allowing developers to use this data in their smart contracts. Only a small subset of data required by a smart contract is published on-chain.

They guarantee that the web data received has not been tampered with. Proving such claims on-chain without Web Proofs is difficult. Especially if we want to implement an automated and trusted solution. 

## Example Prover
Let's say we want to create an influencer DAO (_Decentralized Autonomous Organization_) for content creators who make at least $10,000 a month on  YouTube. 

Below is sample code for such a `Prover` contract:

```solidity
import "openzeppelin/contracts/utils/Strings.sol";

import {Prover} from "vlayer/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer/WebProof.sol";

contract YouTubeRevenue is Prover {
    using Strings for string;
    using WebProofLib for WebProof;
    using WebLib for Web;
    
    public string dataUrl = "https://studio.youtube.com/creator/get_channel_dashboard";
    
    function main(WebProof calldata webProof, address influencerAddr) public returns (address, string) {
      Web memory web = webProof.verify(dataUrl);

      require(
        web.jsonGetInt("channel.estimatedEarnings") > 1_000_000, 
        "Earnings less than $10000"
      );

      return (influencerAddr, webProof.jsonGetString("channel.id"));
    }
} 
```

What happens in the above code?  

* First, we need to set up the `Prover` contract:
  * `YouTubeRevenue` inherits from `Prover` vlayer contract that allows off-chain proving of web data.
  * `main` receives `WebProof` as argument, which contains a transcript of an HTTPS session signed by a *Notary* (see section [Security Considerations](#security-considerations) below for details about TLS *Notary*).

* Then, we need to make sure that the Web Proof is valid - the call `webProof.verify(dataUrl)` performs:
  * verification of the validity of the HTTPS transcript.
  * verification of the signature of the *Notary* who signed the transcript.
  * a check whether the *Notary* is the one we trust (we verify this by checking their key used to sign the data).
  * a check that the HTTPS data comes from a server whose identity (server name specified in the server's SSL certificate) is the one we expect (in this case `studio.youtube.com`, which is the domain name in `dataUrl`).
  * a check whether the HTTPS data comes from the expected `dataUrl`.
  * retrieval of plaintext transcript from the Web Proof and returns it as `Web` for further processing.

* Then we have to ensure that the delivered data makes sense for our case:
  * `web.jsonGetInt("channel.estimatedEarnings") > 1_000_000` [parses JSON body](./regex-and-json.md#json-parsing) of the HTTP response, retrieves the `channel.estimatedEarnings` path of the JSON and checks if estimated earnings are higher than 10k USD (parsed JSON contains amount in cents).

Finally, we can return public input:
* The `influencerAddr` and the `web.jsonGetString("channel.id")` will be returned if all checks have passed.

If no execution errors occurred and the proof has been produced, we are ready for on-chain verification. 

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

Contract below is responsible for adding caller's wallet address to the list of DAO members. 

```solidity
import { YouTubeRevenue } from "./v/YouTubeRevenue.v.sol";

address constant PROVER_ADDR = 0xd7141F4954c0B082b184542B8b3Bd00Dc58F5E05;
bytes4 constant  PROVER_FUNC_SELECTOR = YouTubeRevenue.main.selector;

contract InfluencerDao is Verifier {
  mapping(address => bool) public authorizedMembers; 
  mapping(string => bool) public claimedChannels;

  function join(Proof _p, address influencerAddr, string calldata channelId) 
    public 
    onlyVerified(PROVER_ADDR, PROVER_FUNC_SELECTOR)  
  { 
    require(influencerAddr == msg.sender, "Wrong caller");
    require(!claimedChannels[channelId], "ChannelId already used");

    authorizedMembers[influencerAddr] = true;
    claimedChannels[channelId] = true;
  }
}
```
What exactly was going on in the snippet above?

* First, note that we need to tell the `Verifier` which `Prover` contract to verify:
  * The `PROVER_ADDR` constant holds the address of the `Prover` contract that generated the proof. 
  * The `PROVER_FUNC_SELECTOR` constant holds the selector for the `Prover.main()` function. 
  * `InfluencerDao` inherits from Verifier, so we can call the `onlyVerified` modifier, which ensures that the `Proof` we pass is correct.

> You don't need to pass `Proof` as an argument to `onlyVerified` because it is automatically extracted from `msg.data`.

* Next, we add two fields needed to track DAO members:
  * The `authorizedMembers` mapping holds the addresses of DAO members.
  * The `claimedChannels` mapping holds already claimed channels.

* Finally, we need logic to add new members to the DAO:   
  * `Proof` must be first argument to `join`, such that `onlyVerified` has access to it and can verify it.
  * `influencerAddr == msg.sender` checks if it's the YouTube channel owner who is trying to join the DAO.
  * The `!claimedChannels[channelId]` assertion prevents the same channel from being used more than once.
  * `authorizedMembers[msg.sender] = true` adds new member to DAO.
  * `claimedChannels[channelId] = true` marks `channelId` as a claimed channel.

And that's it! 

As you can see, Web Proofs can be useful for building dApps. 

## Security Considerations

The Web Proof feature is based on the [TLSNotary](https://tlsnotary.org/) protocol. Web data is retrieved from an HTTP endpoint and it's integrity and authenticity during the HTTP session is verified using the Transport Layer Security (TLS) protocol (the "S" in HTTPS), which secures most modern encrypted connections on the Internet. Web Proofs ensure the integrity and authenticity of web data after the HTTPS session finishes by extending the TLS protocol. A designated server, called *Notary*, joins the HTTPS session between the client and the server and can cryptographically certify its contents.

From privacy perspective, it is important to note that the *Notary* server never has access to the plaintext transcript of the connection and therefore, *Notary* can never steal client data and pretend to be client. Furthermore, the transcript can be redacted (i.e. certain parts can be removed) by the client, making these parts of the communication not accessible by `Prover` and vlayer infrastructure running the `Prover`.

### Trust Assumptions

It is important to understand that the *Notary* is a trusted party in the above setup. Since the *Notary* certifies the data, a malicious *Notary* could collude with a malicious client to create fake proofs that would still be successfully verified by `Prover`. Currently vlayer runs it's own *Notary* server, which means that vlayer needs to be trusted to certify HTTPS sessions.

 Currently vlayer also needs to be trusted when passing additional data (data other than the Web Proof itself) to `Prover` smart contract, e.g. `influencerAddr` in the example above. The Web Proof could be hijacked before running `Prover` and additional data, different from the original, could be passed to `Prover`, e.g. an attacker could pass their own address as `influencerAddr` in our `YouTubeRevenue` example. Before going to production this will be addressed by making the setup trustless through an association of the additional data with a particular Web Proof in a way that's impossible to forge.

vlayer will publish a roadmap outlining how it will achieve a high level of security when using the *Notary* service.
