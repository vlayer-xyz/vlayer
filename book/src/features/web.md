# Web 
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
import {WebProof, WebProofLib} from "vlayer/WebProof.sol";

contract YouTubeRevenue is Prover {
    using Strings for string;
    using WebProofLib for WebProof;
    
    public string dataUrl = "https://studio.youtube.com/creator/get_channel_dashboard";
    
    function main(WebProof calldata webProof, address influencerAddr) public returns (address, string) {
      webProof.verify(dataUrl);

      require(
        webProof.json().get("channel.estimatedEarnings") > 1_000_000, 
        "Earnings less than $10000"
      );

      return (influencerAddr, webProof.json().get("channel.id"));
    }
} 
```

What happens in the above code?  

* First, we need to set up the `Prover` contract:
  * `YouTubeRevenue` inherits from `Prover` vlayer contract that allows off-chain proving of web data.
  * `main` receives `WebProof` as argument, which contains a transcript of an HTTPS session signed by a Notary (see section [Security Considerations](#security-considerations) below for details about TLS Notary).

* Then, we need to make sure the Web Proof is valid - the call `webProof.verify(dataUrl)` performs:
  * verification of the validity of the HTTPS transcript.
  * verification of the signature of the Notary who signed the transcript.
  * a check whether the Notary is the one we trust (by checking their key used to sign the data).
  * a check that the HTTPS data comes from a server whose identity (as specified in the server's SSL certificate) is the one we expect (in this case `studio.youtube.com`, which is the domain name in `dataUrl`).
  * a check whether the HTTPS request targeted the expected `dataUrl`.
  * retrieval of plaintext transcript from the Web Proof and makes it available for further `webProof` calls.

* Then we have to ensure that the delivered data makes sense for our case:
  * `web.json()` parses JSON body of the HTTP response and allows subsequent `get()` calls.
  * `web.json().get("channel.estimatedEarnings") > 1_000_000` retrieves the `channel.estimatedEarnings` path of the JSON and checks if estimated earnings are higher than 10k USD (parsed JSON contains amount in cents).

Finally, we can return public input:
* The `influencerAddr` and the `web.json().get("channel.id")` will be returned if all checks have passed.

If no execution errors occured and proof was produced, we are ready for on-chain verification. 

> 💡 **Try it Now**
> 
> To run the above example on your computer, type the following command in your terminal:
> 
> ```bash
> vlayer init --template web_example
> ```
> 
> This command will download all necessary artifacts to your project.

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
vlayer retrieves web data and verifies it using the Transport Layer Security (TLS) protocol, which secures most modern encrypted connections on the Internet. Web Proofs ensures the authenticity of web data by extending the TLS protocol. A designated server, called a *Notary*, joins a TLS session between the client and server and can cryptographically certify its contents.

From the privacy perspective, it is crucial that the *Notary* server never has access to the transcript of the connection.

The web proof feature is based on the [TLSNotary](https://tlsnotary.org/) protocol.

### Notary Trust Assumption
A key disadvantage of using a *Notary* is the requirement to trust it. Since the *Notary* certifies incoming data, a malicious *Notary* could create fake proofs that are still validated by the *Verifier*.

vlayer will publish a roadmap outlining how it will achieve a high level of security when using the *Notary* service.