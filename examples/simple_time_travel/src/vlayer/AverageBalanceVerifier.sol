// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {AverageBalance} from "./AverageBalance.sol";
import {HodlerBadgeNFT} from "./HodlerBadgeNFT.sol";

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Verifier} from "vlayer-0.1.0/Verifier.sol";

contract AverageBalanceVerifier is Verifier {
    address public prover;
    mapping(address => bool) public claimed;
    HodlerBadgeNFT public reward;

    constructor(address _prover) {
        prover = _prover;
        reward = new HodlerBadgeNFT();
    }

    function claim(Proof calldata, address claimer, uint256 average)
        public
        onlyVerified(prover, AverageBalance.averageBalanceOf.selector)
    {
        require(!claimed[claimer], "Already claimed");

        if (average >= 10_000_000) {
            claimed[claimer] = true;
            reward.mint(claimer);
        }
    }
}
