// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { ECDSA } from "openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";
import { MessageHashUtils } from "openzeppelin-contracts/contracts/utils/cryptography/MessageHashUtils.sol";
import { Strings } from "openzeppelin/contracts/utils/Strings.sol";
import { IERC20 } from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import { Prover } from "vlayer/Prover.sol";

contract PrivateAirdropProver is Prover {
    using ECDSA for bytes32;
    using MessageHashUtils for bytes;
    IERC20 public immutable TOKEN;

    constructor(IERC20 token) {
        TOKEN = token;
    }

    function nullifier(address _addr) public view returns (bytes32) {
        return(keccak256(abi.encodePacked(_addr)));
    }

    function main(address account, bytes memory signature) public returns (uint256, bytes32) {
        uint256 balance = TOKEN.balanceOf(account);
        require(balance > 0, "Insufficient balance");

        require(isValidSignature(
            account,
            signature
            ), "Invalid Signature"
        );

        return (balance, nullifier(account));
    }

    function isValidSignature(address _account, bytes memory signature) internal pure returns (bool) {
        require(_account != address(0), "Missing Address");


        // bytes32 signedHash = keccak256(
        //     abi.encodePacked("\x19Ethereum Signed Message:\n", Strings.toString(bytes("erc20 prover").length), "erc20 prover")
        // );

        bytes32 signedHash = bytes("erc20 prover").toEthSignedMessageHash();
        return signedHash.recover(signature) == _account;
    }
}
