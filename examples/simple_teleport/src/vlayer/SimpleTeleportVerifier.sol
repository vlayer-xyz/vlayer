// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {SimpleTeleportProver} from "./SimpleTeleportProver.sol";
import {WhaleBadgeNFT} from "./WhaleBadgeNFT.sol";

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Verifier} from "vlayer-0.1.0/Verifier.sol";

contract SimpleTeleportVerifier is Verifier {
    address public prover;
    mapping(address => bool) public claimed;
    WhaleBadgeNFT public reward;

    constructor(address _prover, WhaleBadgeNFT _nft) {
        prover = _prover;
        reward = _nft;
    }

    function claim(Proof calldata, address claimer, uint256 crossChainBalance)
        public
        onlyVerified(prover, SimpleTeleportProver.crossChainBalanceOf.selector)
    {
        require(!claimed[claimer], "Already claimed");

        if (crossChainBalance >= 10_000_000_000_00) {
            claimed[claimer] = true;
            reward.mint(claimer);
        }
    }
}
