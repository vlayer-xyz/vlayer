// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Simple} from "../src/Simple.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

contract SimpleTest is Test {

    Simple public simple;
    RiscZeroMockVerifier public verifier;

    bytes proof;
    bytes journal;
    bytes32 journalHash;

    function setUp() public {
        verifier = new RiscZeroMockVerifier(bytes4(0));
        simple = new Simple(verifier);

        journal = new bytes(1);
        journal[0] = 0x04;

        journalHash = keccak256(journal);

    }

    function test_updateSum() public {
        simple.updateSum(journalHash, journal, 4);
        assertEq(simple.latestSum(), 4);
    }

}
