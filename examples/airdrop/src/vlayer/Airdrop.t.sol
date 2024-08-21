// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "vlayer/testing/VTest.sol";
import "./NftOwnershipProver.sol";
import "./AirdropVerifier.sol";

contract AirdropTest is VTest {
    function test_airdrop() public {
        NftOwnershipProver prover = new NftOwnershipProver();
        Airdrop airdrop = new Airdrop(address(prover));
        callProver();
        address owner = prover.main(0xaAa2DA255DF9Ee74C7075bCB6D81f97940908A5D);
        Proof memory proof = getProof();
        airdrop.claim(proof, owner);
    }
}