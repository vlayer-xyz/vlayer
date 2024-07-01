// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";

import {Receipt, VerificationFailed} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";
import {ControlID, RiscZeroGroth16Verifier} from "risc0-ethereum/groth16/RiscZeroGroth16Verifier.sol";

import {Steel} from "vlayer/Steel.sol";
import {Simple} from "../src/Simple.sol";

contract SimpleTest is Test {
    Simple public simple;
    RiscZeroMockVerifier public verifier;

    bytes32 public constant GUEST_ID = bytes32(0xb8d08f84d65bc7aadd17445d52f12be026dce5b26587534860b8a7660e8741b4);

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

    uint8[256] s = [
         9, 109, 139, 231, 39, 228, 37, 99, 40, 137, 123, 196, 183, 205, 137, 50, 0, 242, 62, 35, 180, 103, 60, 70, 118, 151, 14, 103, 242, 112, 248, 96, 45, 132, 134, 1, 212, 1, 184, 176, 231, 12, 23, 190, 76, 66, 106, 92, 243, 206, 151, 24, 172, 245, 80, 102, 54, 102, 188, 88, 91, 250, 88, 52, 36, 96, 70, 164, 94, 62, 46, 132, 146, 80, 83, 92, 229, 56, 204, 48, 128, 114, 150, 4, 76, 122, 137, 23, 108, 244, 229, 29, 109, 87, 177, 216, 5, 23, 254, 192, 97, 29, 177, 180, 172, 95, 44, 166, 130, 57, 189, 76, 171, 233, 166, 142, 132, 100, 170, 122, 21, 174, 145, 140, 128, 190, 31, 117, 9, 215, 59, 46, 5, 132, 69, 12, 191, 128, 247, 199, 117, 182, 158, 126, 15, 154, 80, 45, 25, 81, 154, 4, 165, 173, 160, 252, 236, 79, 174, 197, 18, 219, 236, 82, 55, 203, 12, 191, 100, 215, 142, 201, 139, 218, 62, 103, 219, 22, 168, 40, 178, 19, 32, 105, 233, 50, 176, 101, 119, 22, 160, 42, 5, 179, 171, 126, 108, 40, 185, 197, 35, 41, 49, 15, 120, 118, 98, 128, 168, 214, 59, 109, 68, 212, 181, 168, 161, 224, 156, 11, 199, 106, 5, 163, 24, 108, 73, 37, 109, 3, 65, 116, 73, 215, 43, 197, 142, 36, 209, 97, 123, 11, 171, 52, 150, 46, 64, 238, 148, 174, 114, 200, 93, 96, 106, 23
    ];

    bytes journalBytes;
    bytes32 journalHash;

    bytes seal;

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

    function test_updateSum_validatesActualGroth16Proof() public {

        RiscZeroGroth16Verifier groth16Verifier = new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
        simple = new Simple(groth16Verifier);

        for(uint i = 0; i < s.length; i++){
            seal.push(bytes1(s[i]));
        }

        emit logs(seal);

        simple.updateSum(seal, fixture_commitment(), fixture_sum());
        assertEq(simple.latestSum(), 3);
    }

}
