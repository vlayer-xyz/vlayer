// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Proof} from "vlayer-0.1.0/src/Proof.sol";
import {Verifier} from "vlayer-0.1.0/src/Verifier.sol";
import {PrivateAirdropProver} from "./PrivateAirdropProver.sol";
import {IERC20} from "@openzeppelin-contracts-5.0.1/token/ERC20/IERC20.sol";

// This contract is executed on-chain (Ethereum Mainnet, Arbitrum, Base, etc.)
contract PrivateAirdropVerifier is Verifier {
    IERC20 private awesomeToken;
    address private proverContractAddr;
    mapping(bytes32 => bool) public claimedNullifiers;

    constructor(address proverAddr, IERC20 token) {
        proverContractAddr = proverAddr;
        awesomeToken = token;
    }

    function claim(Proof calldata, address sender, bytes32 nullifier)
        public
        onlyVerified(proverContractAddr, PrivateAirdropProver.main.selector)
    {
        require(claimedNullifiers[nullifier] == false, "Already withdrawn");
        claimedNullifiers[nullifier] = true;
        awesomeToken.transfer(sender, 1000);
    }
}
