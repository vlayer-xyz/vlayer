// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import { ERC721 } from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import { VlayerVerifier } from "@vlayer/VlayerVerifier.sol";

interface IERC721 {
    function ownerOf(uint256 tokenId) external view returns (address owner);
}

contract PrivateAirdrop is ERC721("GameItem", "ITM"), VlayerVerifier {
    address public targetNftAddr;
    uint256 public lastClaimedId = 0;

    constructor (address _targetNftAddr) {
        targetNftAddr = _targetNftAddr; // provide address of the ERC721 NFT contract that we are checking, ie. Upcade contract
    } 

    function claim(VProof calldata proof, address receiver) public onlyVerified() {
        // add extra sanity checks here
        lastClaimedId += 1;
        _mint(receiver, lastClaimedId); // deliver Airdrop NFT as reward
    }
}
