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
contract YouTubeRevenue is Prover {
    string memory dataUrl = "https://studio.youtube.com/creator/get_channel_dashboard";
    
    function main(address influencerAddr) public returns (address, string) {      
      require(web.url.equal(dataUrl), "Incorrect URL")
      require(
        web.json.get("channel.estimatedEarnings") > 1_000_000, 
        "Earnings less than $10000"_
      )

      return (influencerAddr, web.json.get("channel.id"));
    }
}
```

What happens in the above code?  
* First, we need to set up the `Prover` contract:
  * `YouTubeRevenue` inherits from `Prover` vlayer contract that allows off-chain proving of web data
  * inside `main` we use the `web` structure, which is injected into contract context by vlayer

Then we have to ensure that the delivered data makes sense for our case: 
* `web.url.equal(dataUrl)` checks if injected payload comes from correct URL 
* `estimatedEarnings > 1_000_000` checks if estimated earnings are higher than 10k USD (parsed JSON contains amount in cents). Otherwise it reverts 

Finally, we can return public input:
* The `influencerAddr` and the `channelId` will be returned if all checks have passed

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
    require(influencerAddr == msg.sender, "wrong caller")
    require(!claimedChannels[channelId], "ChannelId already used")

    authorizedMembers[influencerAddr] = true;
    claimedChannels[channelId] = true;
  }
}
```
What exactly was going on in the snippet above?

* First, note that we need to tell the `Verifier` which `Prover` contract to verify:
  * The `PROVER_ADDR` constant holds the address of the `Prover` contract that generated the proof. 
  * The `PROVER_FUNC_SELECTOR` constant holds the selector for the `Prover.main()` function. 
  * `InfluencerDao` inherits from Verifier, so we can call the `onlyVerified` modifier, which ensures that the `proof` we pass is correct

> You don't need to pass `proof` as an argument to `onlyVerified` because it is automatically extracted from `msg.data`.

* Next, we add two fields needed to track DAO members:
  * The `authorizedMembers` mapping holds the addresses of DAO members.
  * The `claimedChannels` mapping holds already claimed channels.

* Finally, we need logic to add new members to the DAO:   
  * `proof` must be first argument, so `onlyVerified` has access to it and can verify it
  * the `!claimedChannels[channelId]` assertion prevents the same channel from being used more than once
  * `authorizedMembers[influencerAddr] = true` adds new member to DAO
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