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
    address private immutable TOKEN_ADDR;
    mapping(address => bool) public withdrawn;

    constructor(address _prover, address _tokenAddr) {
        PROVER = _prover;
        TOKEN_ADDR = _tokenAddr;
    }

    function claim(Proof calldata, address sender)
        public
        onlyVerified(PROVER, FUNCTION_SELECTOR)
    {
        require(withdrawn[sender] == false, "Already withdrawn");
        withdrawn[sender] = true;
        IAwesomeToken(TOKEN_ADDR).transfer(sender, 1000);
    }
}
