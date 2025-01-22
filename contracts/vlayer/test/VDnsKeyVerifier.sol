// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {IAccessControl} from "@openzeppelin-contracts-5.0.1/access/IAccessControl.sol";

import "forge-std-1.9.4/src/Test.sol";
import {KeyVault, IVDnsKeyVerifier} from "../src/KeyVault.sol";

contract VDnsKeyVerifierTest is Test {
    KeyVault keyVault;

    function setUp() public {
        keyVault = new KeyVault();
        keyVault.grantRole(keyVault.KEY_MANAGER_ROLE(), address(this));
    }

    function test_addKeyMakesKeyValid() public {
        bytes memory key = "0x1234";
        assertFalse(keyVault.isDnsKeyValid(key));

        keyVault.addDnsKey(key);
        assertTrue(keyVault.isDnsKeyValid(key));
    }

    function test_revertsIf_keyIsAlreadyValid() public {
        bytes memory key = "0x1234";
        keyVault.addDnsKey(key);

        vm.expectRevert("Key is already valid");
        keyVault.addDnsKey(key);
    }

    function test_onlyAdminCanAddKey() public {
        bytes memory key = "0x1234";

        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, address(123), keyVault.KEY_MANAGER_ROLE()
            )
        );
        vm.prank(address(123));
        keyVault.addDnsKey(key);
    }

    function test_addKeyEmitsEvent() public {
        bytes memory key = "0x1234";

        vm.expectEmit();
        emit IVDnsKeyVerifier.KeyAdded(address(this), key);
        keyVault.addDnsKey(key);
    }

    function test_revokeKeyMakesKeyInvalid() public {
        bytes memory key = "0x1234";
        keyVault.addDnsKey(key);
        assertTrue(keyVault.isDnsKeyValid(key));

        keyVault.revokeDnsKey(key);
        assertFalse(keyVault.isDnsKeyValid(key));
    }

    function test_onlyAdminCanRevokeKey() public {
        bytes memory key = "0x1234";
        keyVault.addDnsKey(key);

        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, address(123), keyVault.KEY_MANAGER_ROLE()
            )
        );
        vm.prank(address(123));
        keyVault.revokeDnsKey(key);
    }

    function test_revokeKeyEmitsEvent() public {
        bytes memory key = "0x1234";
        keyVault.addDnsKey(key);

        vm.expectEmit();
        emit IVDnsKeyVerifier.KeyRevoked(address(this), key);
        keyVault.revokeDnsKey(key);
    }

    function test_revertsIf_keyIsAlreadyInvalid() public {
        bytes memory key = "0x1234";
        vm.expectRevert("Key is invalid");
        keyVault.revokeDnsKey(key);
    }
}
