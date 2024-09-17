// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Proof} from "vlayer-0.1.0/src/Proof.sol";
import {Verifier} from "vlayer-0.1.0/src/Verifier.sol";

import {ERC20} from "@openzeppelin-contracts-5.0.1/token/ERC20/ERC20.sol";

import {NftOwnershipProver} from "./NftOwnershipProver.sol";

bytes4 constant FUNCTION_SELECTOR = NftOwnershipProver.main.selector;

contract AwesomeToken is ERC20 {
    constructor() ERC20("AwesomeToken", "AT") {
        _mint(msg.sender, 1000000);
    }
}

// This contract is executed on-chain (Ethereum Mainnet, Arbitrum, Base, etc.)
contract Airdrop is Verifier {
    address public immutable PROVER;
    AwesomeToken public immutable TOKEN;
    mapping(address => bool) public withdrawn;

    constructor(address _prover) {
        PROVER = _prover;
        TOKEN = new AwesomeToken();
    }

    function claim(Proof calldata, address sender) public onlyVerified(PROVER, FUNCTION_SELECTOR) {
        require(withdrawn[sender] == false, "Already withdrawn");
        withdrawn[sender] = true;
        TOKEN.transfer(sender, 1000);
    }
}
