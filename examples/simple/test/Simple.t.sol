// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Simple} from "../src/Simple.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

contract SimpleTest is Test {

    Simple public simple;
    RiscZeroMockVerifier public verifier;

    function setUp() public {
        verifier = new RiscZeroMockVerifier(bytes4(0));
        simple = new Simple(verifier);
    }

    function test_updateSum() public {

        Simple.Commitment memory commitment = Simple.Commitment({
            offset: 0,
            length: 0,
            version: 1,
            chainId: 11155111,
            blockNumber: 2,
            blockHash: bytes32(0xcbbeae20657c38f6ae82403a1c5d4e7b27142af11b02bde8bf1e3e93878e451f),
            seal: new bytes(0)
        });
        uint256 sum = 3;
      
        // journal value taken from host execution
        uint8[316] memory journal = [78, 0, 0, 0, 248, 0, 0, 0, 76, 0, 0, 0, 234, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 1, 0, 0, 0, 131, 0, 0, 0, 170, 0, 0, 0, 54, 0, 0, 0, 167, 0, 0, 0, 2, 0, 0, 0, 160, 0, 0, 0, 203, 0, 0, 0, 190, 0, 0, 0, 174, 0, 0, 0, 32, 0, 0, 0, 101, 0, 0, 0, 124, 0, 0, 0, 56, 0, 0, 0, 246, 0, 0, 0, 174, 0, 0, 0, 130, 0, 0, 0, 64, 0, 0, 0, 58, 0, 0, 0, 28, 0, 0, 0, 93, 0, 0, 0, 78, 0, 0, 0, 123, 0, 0, 0, 39, 0, 0, 0, 20, 0, 0, 0, 42, 0, 0, 0, 241, 0, 0, 0, 27, 0, 0, 0, 2, 0, 0, 0, 189, 0, 0, 0, 232, 0, 0, 0, 191, 0, 0, 0, 30, 0, 0, 0, 62, 0, 0, 0, 147, 0, 0, 0, 135, 0, 0, 0, 142, 0, 0, 0, 69, 0, 0, 0, 31, 0, 0, 0, 128, 0, 0, 0, 224, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 3, 0, 0, 0];
        bytes32 journalHash = keccak256(abi.encode(journal));

        simple.updateSum(commitment, sum, journalHash);
        assertEq(simple.latestSum(), 3);
    }

}
