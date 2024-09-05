// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";

import {EmailProofLib, EmailProof} from "../src/EmailProof.sol";

contract EmailProofTest is Test {
    using EmailProofLib for EmailProof;

    function test_verifyReturnsPlainEmail() public {
        EmailProof memory emailProof = EmailProof("email");
        assertEq(emailProof.verify(), "email");
    }
}