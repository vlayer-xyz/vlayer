// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import { Verifier } from "vlayer/Verifier.sol";
import { NftOwnership } from "./NftOwnership.sol";

address constant NFT_OWNERSHIP_VLAYER_CONTRACT = 0x1744aC92e0Ff310Ff836bB68d56D4159E37D0BdF;
bytes4 constant FUNCTION_SELECTOR = NftOwnership.main.selector;

interface IAwesomeToken {
  function transfer(address to, uint256 amount) external;
}

// This contract is executed on-chain (Ethereum Mainnet, Arbitrum, Base, etc.)
contract Airdrop is Verifier {
  address awesomeTokenAddr = 0x510848bE71Eac101a4Eb871C6436178e52210646;
  mapping (address => bool) public withdrawn;

  function claim(Proof calldata proof, address sender)
    public
    onlyVerified(NFT_OWNERSHIP_VLAYER_CONTRACT, FUNCTION_SELECTOR)
  {
    require(withdrawn[sender] == false, "Already withdrawn");

    IAwesomeToken(awesomeTokenAddr).transfer(sender, 1000);
    withdrawn[sender] = true;
  }
}
