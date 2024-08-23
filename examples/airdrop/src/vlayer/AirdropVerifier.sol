// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Proof} from "vlayer/Proof.sol";
import {Verifier} from "vlayer/Verifier.sol";

import {NftOwnershipProver} from "./NftOwnershipProver.sol";

bytes4 constant FUNCTION_SELECTOR = NftOwnershipProver.main.selector;

interface IAwesomeToken {
    function transfer(address to, uint256 amount) external;
}

// This contract is executed on-chain (Ethereum Mainnet, Arbitrum, Base, etc.)
contract Airdrop is Verifier {
    address private immutable PROVER;
    mapping(address => bool) public withdrawn;

    constructor(address _prover) {
        PROVER = _prover;
    }

    function claim(Proof calldata, address sender)
        public
        onlyVerified(PROVER, FUNCTION_SELECTOR)
    {
        require(withdrawn[sender] == false, "Already withdrawn");
        withdrawn[sender] = true;
        IAwesomeToken(awesomeTokenAddr).transfer(sender, 1000);
    }
}
