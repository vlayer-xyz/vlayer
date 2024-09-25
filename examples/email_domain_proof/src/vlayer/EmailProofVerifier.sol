// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {EmailDomainProver} from "./EmailDomainProver.sol";

import {Proof} from "vlayer-0.1.0/src/Proof.sol";
import {Verifier} from "vlayer-0.1.0/src/Verifier.sol";

contract EmailDomainVerifier is Verifier {
    address public prover;

    mapping(bytes32 => address) public emailHashToAddr;

    constructor(address _prover) {
        prover = _prover;
    }

    function verify(Proof calldata, bytes32 _emailHash, address _targetWallet)
        public
        onlyVerified(prover, EmailDomainProver.main.selector)
    {
        emailHashToAddr[_emailHash] = _targetWallet;
    }
}
