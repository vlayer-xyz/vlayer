// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import "../../src/testing/TestVerifierRouterDeployer.sol";
import {Test, console} from "forge-std-1.9.4/src/Test.sol";

contract TestVerifierRouterDeployer_Tests is Test {
    function test_WhitelistsTestingDnsPublicKey() public {
        bytes32[] memory imageIds = new bytes32[](0);

        TestVerifierRouterDeployer deployer = new TestVerifierRouterDeployer(imageIds);
        Repository repository = Repository(address(deployer.VERIFIER_ROUTER().imageIdRepository()));

        assertEq(repository.isDnsKeyValid(TEST_DNS_PUBLIC_KEY), true);
    }

    function test_AddsImageIdToRepository() public {
        bytes32[] memory imageIds = new bytes32[](2);
        imageIds[0] = bytes32(uint256(0x1234));
        imageIds[1] = bytes32(uint256(0x5678));
        TestVerifierRouterDeployer deployer = new TestVerifierRouterDeployer(imageIds);
        Repository repository = Repository(address(deployer.VERIFIER_ROUTER().imageIdRepository()));

        assertEq(repository.isImageSupported(bytes32(uint256(0x1234))), true);
        assertEq(repository.isImageSupported(bytes32(uint256(0x5678))), true);
        assertEq(repository.isImageSupported(bytes32(uint256(0x9876))), false);
    }
}
