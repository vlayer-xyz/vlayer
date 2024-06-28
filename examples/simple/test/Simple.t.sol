// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";

import {Receipt} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

import {Steel} from "vlayer/Steel.sol";
import {Simple} from "../src/Simple.sol";

contract SimpleTest is Test {

    Simple public simple;
    RiscZeroMockVerifier public verifier;

    bytes32 public constant GUEST_ID = bytes32(0xb7079f57c71b4e1d95b8b1254303e13f78914599a8c119534c4c947c996b4d7d);

    // journal value should be taken from host execution
    uint8[160] journal = [
            // address
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe7, 0xf1, 0x72, 0x5e, 0x77, 0x34, 0xce, 0x28, 0x8f, 0x83, 0x67, 0xe1, 0xbb, 0x14, 0x3e, 0x90, 0xbb, 0x3f, 0x05, 0x12,
            // selector
            0xca, 0xd0, 0x89, 0x9b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
            // blockNo
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            // blockHash
            0xcb, 0xbe, 0xae, 0x20, 0x65, 0x7c, 0x38, 0xf6, 0xae, 0x82, 0x40, 0x3a, 0x1c, 0x5d, 0x4e, 0x7b, 0x27, 0x14, 0x2a, 0xf1, 0x1b, 0x02, 0xbd, 0xe8, 0xbf, 0x1e, 0x3e, 0x93, 0x87, 0x8e, 0x45, 0x1f,
            // sum
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03
    ];

    bytes journalBytes;
    bytes32 journalHash;

    function setUp() public {
        verifier = new RiscZeroMockVerifier(bytes4(0));
        simple = new Simple(verifier);

        for (uint32 i = 0; i < journal.length; i++){
            journalBytes.push(bytes1(journal[i]));
        }

        journalHash = keccak256(journalBytes);
    }

    function fixture_sum() private pure returns (uint256) {
        return 3;        
    }

    function fixture_commitment() private pure returns (Steel.ExecutionCommitment memory) {
        return Steel.ExecutionCommitment({
                    startContractAddress: address(0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512),
                    functionSelector: bytes4(0xcad0899b),
                    settleBlockNumber: 2,
                    settleBlockHash: bytes32(0xcbbeae20657c38f6ae82403a1c5d4e7b27142af11b02bde8bf1e3e93878e451f)
                });
    }

    function test_updateSum_validProof() public {

        bytes memory seal = verifier.mockProve(GUEST_ID, journalHash).seal;
        simple.updateSum(seal, fixture_commitment(), fixture_sum());
        assertEq(simple.latestSum(), 3);
    }

    function test_updateSum_revertsForInvalidGuestId() public {
        bytes memory seal = verifier.mockProve(bytes32(0), journalHash).seal;

        vm.expectRevert();
        simple.updateSum(seal, fixture_commitment(), fixture_sum());
    }

    function test_updateSum_revertsForInvalidCalldata() public {
        bytes memory seal = verifier.mockProve(GUEST_ID, journalHash).seal;

        vm.expectRevert();
        simple.updateSum(seal, fixture_commitment(), 4);
    }

}
