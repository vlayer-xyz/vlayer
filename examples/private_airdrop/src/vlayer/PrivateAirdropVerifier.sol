// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import { Proof } from "vlayer/Proof.sol";
import { Verifier } from "vlayer/Verifier.sol";
import { PrivateAirdropProver } from "./PrivateAirdropProver.sol";

interface IAwesomeToken {
    function transfer(address to, uint256 amount) external;
}

// This contract is executed on-chain (Ethereum Mainnet, Arbitrum, Base, etc.)
contract PrivateAirdropVerifier is Verifier {
    address private awesomeTokenAddr;
    address private proverContractAddr;
    mapping(bytes32 => bool) public claimedNullifiers;

    constructor(address proverAddr, IAwesomeToken tokenAddr) {
        proverContractAddr = proverAddr;
        awesomeTokenAddr = address(tokenAddr);
    }

    function claim(Proof calldata, address sender, bytes32 nullifier)
        public
        onlyVerified(proverContractAddr, PrivateAirdropProver.main.selector)
    {
        require(claimedNullifiers[nullifier] == false, "Already withdrawn");
        claimedNullifiers[nullifier] = true;
        IAwesomeToken(awesomeTokenAddr).transfer(sender, 1000);
    }
}
