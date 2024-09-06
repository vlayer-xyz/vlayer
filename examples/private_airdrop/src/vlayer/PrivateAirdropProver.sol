// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ECDSA} from "@openzeppelin-contracts-5.0.2/utils/cryptography/ECDSA.sol";
import {MessageHashUtils} from "@openzeppelin-contracts-5.0.2/utils/cryptography/MessageHashUtils.sol";
import {IERC20} from "@openzeppelin-contracts-5.0.2/token/ERC20/IERC20.sol";
import {Prover} from "vlayer-contracts-0.1.0/src/Prover.sol";

contract PrivateAirdropProver is Prover {
    using ECDSA for bytes32;
    using MessageHashUtils for bytes;
    IERC20 public immutable TOKEN;

    constructor(IERC20 token) {
        TOKEN = token;
    }

    function nullifier(address _addr) public pure returns (bytes32) {
        return (keccak256(abi.encodePacked(_addr)));
    }

    function main(
        address account,
        bytes memory signature
    ) public view returns (address, bytes32) {
        uint256 balance = TOKEN.balanceOf(account);
        require(balance > 0, "Insufficient balance");

        require(isValidSignature(account, signature), "Invalid Signature");

        return (account, nullifier(account));
    }

    function isValidSignature(
        address _account,
        bytes memory signature
    ) internal pure returns (bool) {
        require(_account != address(0), "Missing Address");

        bytes32 signedHash = bytes(
            "I own ExampleToken and I want to privately claim my airdrop"
        ).toEthSignedMessageHash();
        return signedHash.recover(signature) == _account;
    }
}
