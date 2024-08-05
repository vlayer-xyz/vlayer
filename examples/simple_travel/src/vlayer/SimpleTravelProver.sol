// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";

interface IERC20 {
    function owner() external view returns (address);
}

contract SimpleTravelProver is Prover {
    function usdtOwner() public returns (address) {
        address USDT = 0xdAC17F958D2ee523a2206206994597C13D831ec7;
        uint some_block_no = 20_000_000;

        setChain(1, some_block_no);
        return IERC20(USDT).owner();
    }
}
