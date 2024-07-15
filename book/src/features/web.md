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
contract YouTubeRevenue is Prover {
    string memory dataUrl = "https://studio.youtube.com/creator/get_channel_dashboard";
    
    function main() public returns (address, string) {      
      require(web.url.equal(dataUrl), "Incorrect URL")
      require(
        web.json.get("channel.estimatedEarnings") > 1_000_000, 
        "Earnings less than $10000"_
      )

      return (msg.sender, web.json.get("channel.id"));
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
* The caller address (`msg.sender`) and the `channelId` will be returned if all checks have passed

If there were no execution errors occured and proof was produced, we are ready for on-chain verification. 

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

  function join(Proof proof, string calldata channelId) 
    public 
    onlyVerified(PROVER_ADDR, PROVER_FUNC_SELECTOR)  
  { 
    require(!claimedChannels[channelId], "ChannelId already used")

    authorizedMembers[msg.sender] = true;
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
  * `authorizedMembers[msg.sender] = true` adds new member to DAO
  * `claimedChannels[channelId] = true` marks `channelId` as a claimed channel.

And that's it! 

As you can see, Web Proofs can be useful for building dApps. 