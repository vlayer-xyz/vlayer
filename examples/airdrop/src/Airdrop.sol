import { VlayerVerifier } from "@vlayer/VlayerVerifier.sol";

address constant NFT_OWNERSHIP_VLAYER_CONTRACT = 0x1744ac92e0ff310ff836bb68d56d4159e37d0bdf;

// This contract is executed on-chain (Ethereum Mainnet, Arbitrum, Base, etc.)

contract Airdrop is VlayerVerifier {
  address awesomeTokenAddr = 0x510848be71eac101a4eb871c6436178e52210646;
  mapping (address => bool) public withdrawn;

  function drop(Proof proof, address sender) onlyVerified(NFT_OWNERSHIP_CONTRACT) {
    require(withdrawn[sender] == false, "Already withdrawn");
    
    AwesomeToken.transfer(sender, 1000);
    withdrawn[sender] = true;
  }
}
