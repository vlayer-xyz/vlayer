// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";

struct Erc20Token {
    address addr;
    uint256 chainId;
    uint256 blockNumber;
}

contract SimpleTeleportProver is Prover {
    function crossChainBalanceOf(address _owner, Erc20Token[] memory tokens)
        public
        returns (Proof memory, address, uint256[] memory, Erc20Token[] memory)
    {
        uint256[] memory balances = new uint256[](tokens.length);

        for (uint256 i = 0; i < tokens.length; i++) {
            setChain(tokens[i].chainId, tokens[i].blockNumber);
            balances[i] = IERC20(tokens[i].addr).balanceOf(_owner);
        }

        return (proof(), _owner, balances, tokens);
    }
}
