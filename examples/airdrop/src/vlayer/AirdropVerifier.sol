// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Proof} from "vlayer/Proof.sol";
import {Verifier} from "vlayer/Verifier.sol";

import {NftOwnershipProver} from "./NftOwnershipProver.sol";
import { IERC20 } from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";

bytes4 constant FUNCTION_SELECTOR = NftOwnershipProver.main.selector;

interface IAwesomeToken {
    function transfer(address to, uint256 amount) external;
}

// This contract is executed on-chain (Ethereum Mainnet, Arbitrum, Base, etc.)
contract Airdrop is Verifier {
    address private immutable PROVER;
    IERC20 private immutable TOKEN;
    mapping(address => bool) public withdrawn;

    constructor(address _prover, IERC20 _token) {
        PROVER = _prover;
        TOKEN = _token;
    }

    function claim(Proof calldata, address sender)
        public
        onlyVerified(PROVER, FUNCTION_SELECTOR)
    {
        require(withdrawn[sender] == false, "Already withdrawn");
        withdrawn[sender] = true;
        TOKEN.transfer(sender, 1000);
    }
}
