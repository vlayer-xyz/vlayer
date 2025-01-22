// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console} from "forge-std-1.9.4/src/Test.sol";

import {ImageID} from "../../src/ImageID.sol";
import {ImageIdRepository} from "../../src/proof_verifier/ImageIdRepository.sol";

bytes32 constant MOCK_IMAGE_ID = bytes32(0x1111111111111111111111111111111111111111111111111111111111111111);

contract ImageIdRepository_addSupport_Tests is Test {
    ImageIdRepository public repository;

    function setUp() public {
        repository = new ImageIdRepository();
    }

    function test_byDefaultCurrentImageIdIsNotSupported() public view {
        assertTrue(!repository.isSupported(MOCK_IMAGE_ID));
        assertTrue(!repository.isSupported(ImageID.RISC0_CALL_GUEST_ID));
    }

    function test_onceAddedSupportImageIsSupported() public {
        assertTrue(!repository.isSupported(MOCK_IMAGE_ID));

        repository.addSupport(MOCK_IMAGE_ID);

        assertTrue(repository.isSupported(MOCK_IMAGE_ID));
    }

    function test_canSupportMultipleImageIds() public {
        repository.addSupport(MOCK_IMAGE_ID);
        repository.addSupport(bytes32(0x2222222222222222222222222222222222222222222222222222222222222222));

        assertTrue(repository.isSupported(MOCK_IMAGE_ID));
        assertTrue(repository.isSupported(bytes32(0x2222222222222222222222222222222222222222222222222222222222222222)));
    }

    function test_cannotAddSupportForAnAlreadySupportedImageId() public {
        repository.addSupport(MOCK_IMAGE_ID);

        vm.expectRevert("ImageID is already supported");
        repository.addSupport(MOCK_IMAGE_ID);
    }

    function test_emitsNewImageIdEvent() public {
        vm.expectEmit();
        emit ImageIdRepository.AddedImageIDSupport(MOCK_IMAGE_ID);
        repository.addSupport(MOCK_IMAGE_ID);
    }
}

contract ImageIdRepository_revokeSupport_Tests is Test {
    ImageIdRepository public repository;

    function setUp() public {
        repository = new ImageIdRepository();
    }

    function test_canRevokeSupport() public {
        repository.addSupport(MOCK_IMAGE_ID);

        repository.revokeSupport(MOCK_IMAGE_ID);
        assertTrue(!repository.isSupported(MOCK_IMAGE_ID));
    }

    function test_failsToRevokeForUnsupportedImageId() public {
        vm.expectRevert("Cannot revoke unsupported ImageID");
        repository.revokeSupport(MOCK_IMAGE_ID);
    }

    function test_emitsNewImageIdEvent() public {
        repository.addSupport(MOCK_IMAGE_ID);

        vm.expectEmit();
        emit ImageIdRepository.RevokedImageIDSupport(MOCK_IMAGE_ID);
        repository.revokeSupport(MOCK_IMAGE_ID);
    }

    function test_canAddSupportAgainOnceRevoked() public {
        repository.addSupport(MOCK_IMAGE_ID);
        repository.revokeSupport(MOCK_IMAGE_ID);

        repository.addSupport(MOCK_IMAGE_ID);
        assertTrue(repository.isSupported(MOCK_IMAGE_ID));
    }
}
