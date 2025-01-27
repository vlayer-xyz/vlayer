// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console} from "forge-std-1.9.4/src/Test.sol";
import {IAccessControl} from "openzeppelin-contracts/access/IAccessControl.sol";

import {ImageID} from "../../src/ImageID.sol";
import {ImageIdRepository} from "../../src/proof_verifier/ImageIdRepository.sol";

bytes32 constant MOCK_IMAGE_ID = bytes32(0x1111111111111111111111111111111111111111111111111111111111111111);
address constant deployer = address(0);
address constant owner = address(1);
address constant alice = address(2);
address constant bob = address(3);

contract ImageIdRepository_addSupport_Tests is Test {
    ImageIdRepository public repository;

    function setUp() public {
        repository = new ImageIdRepository(address(this), address(this));
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
        repository = new ImageIdRepository(address(this), address(this));
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

contract ImageIdRepository_AdminRole is Test {
    ImageIdRepository public repository;

    bytes32 public immutable ADMIN_ROLE;
    bytes32 public immutable OWNER_ROLE;

    constructor() {
        // deployed only to get DEFAULT_ADMIN_ROLE value, so that is acts as a constant within the tests
        ImageIdRepository tmpRepository = new ImageIdRepository(address(this), address(this));
        ADMIN_ROLE = tmpRepository.DEFAULT_ADMIN_ROLE();
        OWNER_ROLE = tmpRepository.OWNER_ROLE();
    }

    function setUp() public {
        vm.startPrank(deployer);
        repository = new ImageIdRepository(deployer, deployer);
        repository.addSupport(MOCK_IMAGE_ID);
        vm.stopPrank();
    }

    function test_deployerIsByDefaultAnAdmin() public view {
        assertTrue(repository.hasRole(ADMIN_ROLE, deployer));
    }

    function test_deployerCanTransferAdminRoleToOtherAddress() public {
        assertTrue(!repository.hasRole(ADMIN_ROLE, alice));

        vm.prank(deployer);
        repository.transferAdminRole(alice);

        assertTrue(repository.hasRole(ADMIN_ROLE, alice));
    }

    function test_nonAdminCannotTransferAdminRole() public {
        vm.prank(alice);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.transferAdminRole(bob);
    }

    function test_onceTransferedAdminRoleDeployerIsNoLongerAnAdmin() public {
        vm.prank(deployer);
        repository.transferAdminRole(alice);
        assertTrue(!repository.hasRole(ADMIN_ROLE, deployer));
    }

    function test_adminCanGrantOwnerRole() public {
        assertTrue(!repository.hasRole(OWNER_ROLE, owner));

        vm.prank(deployer);
        repository.transferOwnership(owner);

        assertTrue(repository.hasRole(OWNER_ROLE, owner));
    }

    function test_onceOwnershipTransferredPreviousOwnerIsNoLongerAnOwner() public {
        assertTrue(repository.hasRole(OWNER_ROLE, deployer));

        vm.prank(deployer);
        repository.transferOwnership(owner);

        assertTrue(!repository.hasRole(OWNER_ROLE, deployer));
    }

    function test_canTransferOwnershipMultipleTimes() public {
        vm.startPrank(deployer);
        repository.transferOwnership(owner);
        repository.transferOwnership(alice);
        repository.transferOwnership(bob);
        vm.stopPrank();

        assertTrue(!repository.hasRole(OWNER_ROLE, deployer));
        assertTrue(!repository.hasRole(OWNER_ROLE, owner));
        assertTrue(!repository.hasRole(OWNER_ROLE, alice));
        assertTrue(repository.hasRole(OWNER_ROLE, bob));
    }

    function test_adminCannotAddSupportForImageId() public {
        vm.prank(deployer);
        repository.transferOwnership(owner);

        vm.prank(deployer);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.addSupport(bytes32(uint256(2)));
    }

    function test_adminCannotRevokeImageId() public {
        vm.prank(deployer);
        repository.transferOwnership(owner);

        vm.prank(deployer);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.revokeSupport(MOCK_IMAGE_ID);
    }
}

contract ImageIdRepository_OwnerRole is Test {
    ImageIdRepository public repository;

    function setUp() public {
        vm.startPrank(deployer);
        repository = new ImageIdRepository(deployer, owner);
        vm.stopPrank();
    }

    function test_ownerCannotTransferAdminRole() public {
        vm.prank(owner);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.transferAdminRole(alice);
    }

    function test_ownerCanAddSupportForImageId() public {
        vm.prank(owner);
        repository.addSupport(MOCK_IMAGE_ID);
        assertTrue(repository.isSupported(MOCK_IMAGE_ID));
    }

    function test_ownerCanRevokeSupportForImageId() public {
        vm.prank(owner);
        repository.addSupport(MOCK_IMAGE_ID);

        vm.prank(owner);
        repository.revokeSupport(MOCK_IMAGE_ID);

        assertTrue(!repository.isSupported(MOCK_IMAGE_ID));
    }

    function test_nonOwnerCannotAddSupportForImageId() public {
        vm.prank(alice);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.addSupport(MOCK_IMAGE_ID);

        assertTrue(!repository.isSupported(MOCK_IMAGE_ID));
    }

    function test_ownerCannotTransferOwnership() public {
        vm.prank(owner);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.transferOwnership(alice);
    }

    function test_nonOwnerCannotRevokeImageId() public {
        vm.prank(deployer);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.revokeSupport(MOCK_IMAGE_ID);

        vm.prank(alice);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.revokeSupport(MOCK_IMAGE_ID);
    }
}
