// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {VTest, Proof} from "vlayer/testing/VTest.sol";
import {NftOwnershipProver} from "./NftOwnershipProver.sol";
import {ERC20} from "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

import {Airdrop} from "./AirdropVerifier.sol";

contract AirdropTest is VTest {
    function test_airdrop() public {
        address holder = address(0xaAa2DA255DF9Ee74C7075bCB6D81f97940908A5D);
        NftOwnershipProver prover = new NftOwnershipProver();
        Airdrop airdrop = new Airdrop(address(prover));
        callProver();
        address owner = prover.main(holder);
        Proof memory proof = getProof();
        airdrop.claim(proof, owner);
    }
}
