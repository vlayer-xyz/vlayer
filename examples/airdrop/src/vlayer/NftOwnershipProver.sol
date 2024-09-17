// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

// Below contract is executed off-chain to generate proof that the prover owns certain NFT.
// tokenId and msg.sender would be privately provided to the prover
// Such proof can be used by on-chain smart contract to claim airdrop or any other logic

import {Prover} from "vlayer-0.1.0/src/Prover.sol";

interface IERC721 {
    function balanceOf(address owner) external view returns (uint256 balance);
}

address constant BYAC_NFT_ADDR = 0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D;

contract NftOwnershipProver is Prover {
    function require_byac_nft(address owner) public view {
        // Terminate proving if NFT is not owned by the prover
        require(IERC721(BYAC_NFT_ADDR).balanceOf(owner) > 0, "You are not owning any specified NFT");
    }

    function main(address owner) public pure returns (address) {
        // // 🔥 Teleport to chain on which the verification is happening
        //        setChain(1, 20_000_000);

        //        require_byac_nft(owner);

        // anything returned here would be visible to the public
        return owner;
    }
}
