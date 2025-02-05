// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ProofVerifierRouter, FakeProofVerifier, Groth16ProofVerifier} from "../proof_verifier/ProofVerifierRouter.sol";
import {Repository} from "../Repository.sol";

bytes constant TEST_DNS_PUBLIC_KEY =
    hex"30820122300d06092a864886f70d01010105000382010f003082010a0282010100e41b913c0e5e78a84fec1ec6f289036d3ce7737e523e0ecf6b8bb9b08ff95d776c96838b9e702e89e99ebe75ed6812fed63f14fb2591ebab0e940e8a89537de2304643026022d313b38e658197e6526d0bee27bc60fc5a822baeefe9934406ed6d186620676c64da4426e3233d0a3fc118a4c905adc5e539a6ad995cd07d1ed8c96f3a9dbe236ce05b2e01b916e467a30fcee90c4006dc101de818f1003ae21b1e00602ff5dc0c6f80f5153bdf2df1a23068c598434e86cc31585311cd62aa647e6082feaecea25f804a3fcc487fec2bb7feb610027750dd0b88ac65860600887a156ef705761ff11eea53835530ccd4f9b0f8e6dd308217f39c1edcc70ee65d0203010001";

contract TestVerifierRouterDeployer {
    ProofVerifierRouter public immutable VERIFIER_ROUTER;

    constructor(bytes32[] memory imageIds) {
        Repository repository = new Repository(address(this), address(this));
        for (uint256 i = 0; i < imageIds.length; i++) {
            repository.addImageIdSupport(imageIds[i]);
        }
        repository.addDnsKey(TEST_DNS_PUBLIC_KEY);
        repository.transferOwnership(msg.sender);
        repository.transferAdminRole(msg.sender);

        VERIFIER_ROUTER =
            new ProofVerifierRouter(new FakeProofVerifier(repository), new Groth16ProofVerifier(repository));
    }
}
