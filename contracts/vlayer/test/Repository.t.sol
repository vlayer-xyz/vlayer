// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console} from "forge-std-1.9.4/src/Test.sol";
import {IAccessControl} from "@openzeppelin-contracts-5.0.1/access/IAccessControl.sol";

import {ImageID} from "../src/ImageID.sol";
import {Repository, IImageIdRepository, IVDnsKeyRepository, INotaryKeyRepository} from "../src/Repository.sol";

bytes32 constant MOCK_IMAGE_ID = bytes32(0x1111111111111111111111111111111111111111111111111111111111111111);
address constant deployer = address(0);
address constant owner = address(1);
address constant alice = address(2);
address constant bob = address(3);

contract Repository_addImageIdSupport_Tests is Test {
    Repository public repository;

    function setUp() public {
        repository = new Repository(address(this), address(this));
    }

    function test_byDefaultCurrentImageIdIsNotSupported() public view {
        assertTrue(!repository.isImageSupported(MOCK_IMAGE_ID));
        assertTrue(!repository.isImageSupported(ImageID.RISC0_CALL_GUEST_ID));
    }

    function test_onceAddedSupportImageIsSupported() public {
        assertTrue(!repository.isImageSupported(MOCK_IMAGE_ID));

        repository.addImageIdSupport(MOCK_IMAGE_ID);

        assertTrue(repository.isImageSupported(MOCK_IMAGE_ID));
    }

    function test_canSupportMultipleImageIds() public {
        repository.addImageIdSupport(MOCK_IMAGE_ID);
        repository.addImageIdSupport(bytes32(0x2222222222222222222222222222222222222222222222222222222222222222));

        assertTrue(repository.isImageSupported(MOCK_IMAGE_ID));
        assertTrue(
            repository.isImageSupported(bytes32(0x2222222222222222222222222222222222222222222222222222222222222222))
        );
    }

    function test_cannotAddSupportForAnAlreadySupportedImageId() public {
        repository.addImageIdSupport(MOCK_IMAGE_ID);

        vm.expectRevert("ImageID is already supported");
        repository.addImageIdSupport(MOCK_IMAGE_ID);
    }

    function test_emitsNewImageIdEvent() public {
        vm.expectEmit();
        emit IImageIdRepository.ImageIDAdded(MOCK_IMAGE_ID);
        repository.addImageIdSupport(MOCK_IMAGE_ID);
    }
}

contract Repository_revokeImageIdSupport_Tests is Test {
    Repository public repository;

    function setUp() public {
        repository = new Repository(address(this), address(this));
    }

    function test_canRevokeSupport() public {
        repository.addImageIdSupport(MOCK_IMAGE_ID);

        repository.revokeImageIdSupport(MOCK_IMAGE_ID);
        assertTrue(!repository.isImageSupported(MOCK_IMAGE_ID));
    }

    function test_failsToRevokeForUnsupportedImageId() public {
        vm.expectRevert("Cannot revoke unsupported ImageID");
        repository.revokeImageIdSupport(MOCK_IMAGE_ID);
    }

    function test_emitsNewImageIdEvent() public {
        repository.addImageIdSupport(MOCK_IMAGE_ID);

        vm.expectEmit();
        emit IImageIdRepository.ImageIDRevoked(MOCK_IMAGE_ID);
        repository.revokeImageIdSupport(MOCK_IMAGE_ID);
    }

    function test_canAddSupportAgainOnceRevoked() public {
        repository.addImageIdSupport(MOCK_IMAGE_ID);
        repository.revokeImageIdSupport(MOCK_IMAGE_ID);

        repository.addImageIdSupport(MOCK_IMAGE_ID);
        assertTrue(repository.isImageSupported(MOCK_IMAGE_ID));
    }
}

contract Repository_AdminRole is Test {
    Repository public repository;

    bytes32 public immutable ADMIN_ROLE;
    bytes32 public immutable OWNER_ROLE;

    constructor() {
        // deployed only to get DEFAULT_ADMIN_ROLE value, so that is acts as a constant within the tests
        Repository tmpRepository = new Repository(address(this), address(this));
        ADMIN_ROLE = tmpRepository.DEFAULT_ADMIN_ROLE();
        OWNER_ROLE = tmpRepository.OWNER_ROLE();
    }

    function setUp() public {
        vm.startPrank(deployer);
        repository = new Repository(deployer, deployer);
        repository.addImageIdSupport(MOCK_IMAGE_ID);
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
        repository.addImageIdSupport(bytes32(uint256(2)));
    }

    function test_adminCannotRevokeImageId() public {
        vm.prank(deployer);
        repository.transferOwnership(owner);

        vm.prank(deployer);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.revokeImageIdSupport(MOCK_IMAGE_ID);
    }
}

contract Repository_OwnerRole is Test {
    Repository public repository;

    function setUp() public {
        vm.startPrank(deployer);
        repository = new Repository(deployer, owner);
        vm.stopPrank();
    }

    function test_ownerCannotTransferAdminRole() public {
        vm.prank(owner);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.transferAdminRole(alice);
    }

    function test_ownerCanAddSupportForImageId() public {
        vm.prank(owner);
        repository.addImageIdSupport(MOCK_IMAGE_ID);
        assertTrue(repository.isImageSupported(MOCK_IMAGE_ID));
    }

    function test_ownerCanRevokeSupportForImageId() public {
        vm.prank(owner);
        repository.addImageIdSupport(MOCK_IMAGE_ID);

        vm.prank(owner);
        repository.revokeImageIdSupport(MOCK_IMAGE_ID);

        assertTrue(!repository.isImageSupported(MOCK_IMAGE_ID));
    }

    function test_nonOwnerCannotAddSupportForImageId() public {
        vm.prank(alice);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.addImageIdSupport(MOCK_IMAGE_ID);

        assertTrue(!repository.isImageSupported(MOCK_IMAGE_ID));
    }

    function test_ownerCannotTransferOwnership() public {
        vm.prank(owner);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.transferOwnership(alice);
    }

    function test_nonOwnerCannotRevokeImageId() public {
        vm.prank(deployer);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.revokeImageIdSupport(MOCK_IMAGE_ID);

        vm.prank(alice);
        vm.expectPartialRevert(IAccessControl.AccessControlUnauthorizedAccount.selector);
        repository.revokeImageIdSupport(MOCK_IMAGE_ID);
    }
}

contract Repository_DnsKeys is Test {
    Repository public repository;

    function setUp() public {
        repository = new Repository(deployer, owner);
        vm.startPrank(owner);
    }

    function test_addKeyMakesKeyValid() public {
        bytes memory key = "0x1234";
        assertFalse(repository.isDnsKeyValid(key));

        repository.addDnsKey(key);
        assertTrue(repository.isDnsKeyValid(key));
    }

    function test_revertsIf_keyIsAlreadyValid() public {
        bytes memory key = "0x1234";
        repository.addDnsKey(key);

        vm.expectRevert("Key is already valid");
        repository.addDnsKey(key);
    }

    function test_onlyOwnerCanAddKey() public {
        bytes memory key = "0x1234";

        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, alice, repository.OWNER_ROLE()
            )
        );
        vm.startPrank(alice);
        repository.addDnsKey(key);
    }

    function test_addKeyEmitsEvent() public {
        bytes memory key = "0x1234";

        vm.expectEmit();
        emit IVDnsKeyRepository.DnsKeyAdded(owner, key);
        repository.addDnsKey(key);
    }

    function test_revokeKeyMakesKeyInvalid() public {
        bytes memory key = "0x1234";
        repository.addDnsKey(key);
        assertTrue(repository.isDnsKeyValid(key));

        repository.revokeDnsKey(key);
        assertFalse(repository.isDnsKeyValid(key));
    }

    function test_onlyOwnerCanRevokeKey() public {
        bytes memory key = "0x1234";
        repository.addDnsKey(key);

        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, alice, repository.OWNER_ROLE()
            )
        );
        vm.startPrank(alice);
        repository.revokeDnsKey(key);
    }

    function test_revokeKeyEmitsEvent() public {
        bytes memory key = "0x1234";
        repository.addDnsKey(key);

        vm.expectEmit();
        emit IVDnsKeyRepository.DnsKeyRevoked(owner, key);
        repository.revokeDnsKey(key);
    }

    function test_revertsIf_keyIsAlreadyInvalid() public {
        bytes memory key = "0x1234";
        vm.expectRevert("Cannot revoke invalid key");
        repository.revokeDnsKey(key);
    }
}

contract Repository_NotaryKeys is Test {
    Repository public repository;
    string private constant NOTARY_PUB_KEY =
        "-----BEGIN PUBLIC KEY-----\nMFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEe0jxnBObaIj7Xjg6TXLCM1GG/VhY5650\nOrS/jgcbBufo/QDfFvL/irzIv1JSmhGiVcsCHCwolhDXWcge7v2IsQ==\n-----END PUBLIC KEY-----\n";

    function setUp() public {
        repository = new Repository(deployer, owner);
        vm.startPrank(owner);
    }

    function test_addKeyMakesKeyCorrect() public {
        assertFalse(repository.isNotaryKeyValid(NOTARY_PUB_KEY));

        repository.addNotaryKey(NOTARY_PUB_KEY);
        assertTrue(repository.isNotaryKeyValid(NOTARY_PUB_KEY));
    }

    function test_revertsIf_keyIsAlreadyCorrect() public {
        repository.addNotaryKey(NOTARY_PUB_KEY);

        vm.expectRevert("Key is already valid");
        repository.addNotaryKey(NOTARY_PUB_KEY);
    }

    function test_onlyOwnerCanAddKey() public {
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, alice, repository.OWNER_ROLE()
            )
        );
        vm.startPrank(alice);
        repository.addNotaryKey(NOTARY_PUB_KEY);
    }

    function test_addKeyEmitsEvent() public {
        vm.expectEmit();
        emit INotaryKeyRepository.NotaryKeyAdded(owner, NOTARY_PUB_KEY);
        repository.addNotaryKey(NOTARY_PUB_KEY);
    }

    function test_revokeKeyMakesKeyInvalid() public {
        repository.addNotaryKey(NOTARY_PUB_KEY);
        assertTrue(repository.isNotaryKeyValid(NOTARY_PUB_KEY));

        repository.revokeNotaryKey(NOTARY_PUB_KEY);
        assertFalse(repository.isNotaryKeyValid(NOTARY_PUB_KEY));
    }

    function test_onlyOwnerCanRevokeKey() public {
        repository.addNotaryKey(NOTARY_PUB_KEY);

        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, alice, repository.OWNER_ROLE()
            )
        );
        vm.startPrank(alice);
        repository.revokeNotaryKey(NOTARY_PUB_KEY);
    }

    function test_revokeKeyEmitsEvent() public {
        repository.addNotaryKey(NOTARY_PUB_KEY);

        vm.expectEmit();
        emit INotaryKeyRepository.NotaryKeyRevoked(owner, NOTARY_PUB_KEY);
        repository.revokeNotaryKey(NOTARY_PUB_KEY);
    }

    function test_revertsIf_revokingInvalidKey() public {
        vm.expectRevert("Cannot revoke invalid key");
        repository.revokeNotaryKey(NOTARY_PUB_KEY);
    }
}
