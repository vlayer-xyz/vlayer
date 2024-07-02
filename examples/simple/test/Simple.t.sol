// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";

import {Receipt, VerificationFailed} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

import {Steel} from "vlayer-engine/Steel.sol";
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
            0x76, 0x67, 0xa5, 0x94, 0x89, 0x83, 0x15, 0x37, 0x06, 0x34, 0x21, 0x15, 0x0e, 0xfb, 0x68, 0x3f, 0x48, 0x24, 0x98, 0x89, 0x5f, 0x81, 0x19, 0x54, 0x7a, 0x2c, 0xc9, 0xf6, 0x1f, 0xca, 0x60, 0x50,
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
                    settleBlockHash: bytes32(0x7667a59489831537063421150efb683f482498895f8119547a2cc9f61fca6050)
                });
    }

    function test_updateSum_validProof() public {

        bytes memory seal = verifier.mockProve(GUEST_ID, journalHash).seal;
        simple.updateSum(seal, fixture_commitment(), fixture_sum());
        assertEq(simple.latestSum(), 3);
    }

    function test_updateSum_revertsForInvalidGuestId() public {
        bytes memory seal = verifier.mockProve(bytes32(0), journalHash).seal;

        vm.expectRevert(VerificationFailed.selector);
        simple.updateSum(seal, fixture_commitment(), fixture_sum());
    }

    function test_updateSum_revertsForInvalidCalldata() public {
        bytes memory seal = verifier.mockProve(GUEST_ID, journalHash).seal;

        vm.expectRevert(VerificationFailed.selector);
        simple.updateSum(seal, fixture_commitment(), 4);
    }

}
