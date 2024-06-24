// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import { ERC721 } from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import { Airdrop } from "../src/Airdrop.sol";

contract TargetNft is ERC721 {
    constructor() ERC721("GameItem", "ITM") {}

    function mint(address to, uint256 tokenId) public {
        _mint(to, tokenId);
    }
}

contract AirdropTest is Test {
    TargetNft public exampleNft;
    Airdrop public aidrop;
    
    address public alice = address(0x1);
    address public joe = address(0x2);

    function setUp() public {
        exampleNft = new TargetNft();
        aidrop = new Airdrop(address(exampleNft));
    }

    function test_targetNftAddr() public view {
        assertEq(aidrop.targetNftAddr(), address(exampleNft));
    }

    function test_claim() public {
        vm.startPrank(alice);
        exampleNft.mint(alice, 1);
        aidrop.claim(1);
        assertEq(aidrop.ownerOf(1), alice);
        vm.stopPrank();
    }

    function test_claim_reverts() public {
        vm.startPrank(alice);
        exampleNft.mint(joe, 1);
        vm.expectRevert(bytes("You are not the owner of the given NFT"));
        aidrop.claim(1);
        vm.stopPrank();
    }
}
