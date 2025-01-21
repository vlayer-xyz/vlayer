// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {IAccessControl} from "@openzeppelin-contracts-5.0.1/access/IAccessControl.sol";

import "forge-std-1.9.4/src/Test.sol";
import {VDnsKeyVerifier, IVDnsKeyVerifier} from "../src/VDnsKeyVerifier.sol";

contract VDnsKeyVerifierTest is Test {
    VDnsKeyVerifier verifier;

    function setUp() public {
        verifier = new VDnsKeyVerifier();
        verifier.grantRole(verifier.KEY_MANAGER_ROLE(), address(this));
    }

    function test_addKeyMakesKeyValid() public {
        bytes memory key = "0x1234";
        assertFalse(verifier.isKeyValid(key));

        verifier.addKey(key);
        assertTrue(verifier.isKeyValid(key));
    }

    function test_revertsIf_keyIsAlreadyValid() public {
        bytes memory key = "0x1234";
        verifier.addKey(key);

        vm.expectRevert("Key is already valid");
        verifier.addKey(key);
    }

    function test_onlyAdminCanAddKey() public {
        bytes memory key = "0x1234";

        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, address(123), verifier.KEY_MANAGER_ROLE())
        );
        vm.prank(address(123));
        verifier.addKey(key);
    }

    function test_addKeyEmitsEvent() public {
        bytes memory key = "0x1234";

        vm.expectEmit();
        emit IVDnsKeyVerifier.KeyAdded(address(this), key);
        verifier.addKey(key);
    }

    function test_revokeKeyMakesKeyInvalid() public {
        bytes memory key = "0x1234";
        verifier.addKey(key);
        assertTrue(verifier.isKeyValid(key));

        verifier.revokeKey(key);
        assertFalse(verifier.isKeyValid(key));
    }

    function test_onlyAdminCanRevokeKey() public {
        bytes memory key = "0x1234";
        verifier.addKey(key);

        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, address(123), verifier.KEY_MANAGER_ROLE())
        );
        vm.prank(address(123));
        verifier.revokeKey(key);
    }

    function test_revokeKeyEmitsEvent() public {
        bytes memory key = "0x1234";
        verifier.addKey(key);

        vm.expectEmit();
        emit IVDnsKeyVerifier.KeyRevoked(address(this), key);
        verifier.revokeKey(key);
    }

    function test_revertsIf_keyIsAlreadyInvalid() public {
        bytes memory key = "0x1234";
        vm.expectRevert("Key is invalid");
        verifier.revokeKey(key);
    }
}
