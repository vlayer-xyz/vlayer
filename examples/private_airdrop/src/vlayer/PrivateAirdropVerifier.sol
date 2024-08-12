// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import { Proof } from "vlayer/Proof.sol";
import { Verifier } from "vlayer/Verifier.sol";

import { PrivateAirdropProver } from "./PrivateAirdropProver.sol";

address constant PROVER_CONTRACT_ADDR = 0x1744aC92e0Ff310Ff836bB68d56D4159E37D0BdF;
bytes4 constant FUNCTION_SELECTOR = PrivateAirdropProver.prove.selector;

interface IAwesomeToken {
    function transfer(address to, uint256 amount) external;
}

// This contract is executed on-chain (Ethereum Mainnet, Arbitrum, Base, etc.)
contract PrivateAirdropVerifier is Verifier {
    address awesomeTokenAddr = 0x510848bE71Eac101a4Eb871C6436178e52210646;
    mapping(bytes32 => bool) public claimedNullifiers;

    function claim(Proof calldata, address sender, bytes32 nullifier)
        public
        onlyVerified(PROVER_CONTRACT_ADDR, FUNCTION_SELECTOR)
    {
        require(claimedNullifiers[nullifier] == false, "Already withdrawn");

        IAwesomeToken(awesomeTokenAddr).transfer(sender, 1000);
        claimedNullifiers[nullifier] = true;
    }
}
