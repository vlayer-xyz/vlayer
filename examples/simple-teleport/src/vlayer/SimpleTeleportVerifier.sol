// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {SimpleTeleportProver} from "./SimpleTeleportProver.sol";
import {WhaleBadgeNFT} from "./WhaleBadgeNFT.sol";

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Verifier} from "vlayer-0.1.0/Verifier.sol";

struct Erc20Token {
    address addr;
    uint256 chainId;
    uint256 blockNumber;
}

contract SimpleTeleportVerifier is Verifier {
    address public prover;
    mapping(address => bool) public claimed;
    WhaleBadgeNFT public reward;

    constructor(address _prover, WhaleBadgeNFT _nft) {
        prover = _prover;
        reward = _nft;
    }

    function claim(Proof calldata, address claimer, uint256[] calldata balances, Erc20Token[] calldata)
        public
        onlyVerified(prover, SimpleTeleportProver.crossChainBalanceOf.selector)
    {
        require(!claimed[claimer], "Already claimed");

        if (balances.length > 0) {
            uint256 totalBalance = 0;
            for (uint256 i = 0; i < balances.length; i++) {
                totalBalance += balances[i];
            }
            if (totalBalance >= 10_000_000_000_000) {
                claimed[claimer] = true;
                reward.mint(claimer);
            }
        }
    }
}
