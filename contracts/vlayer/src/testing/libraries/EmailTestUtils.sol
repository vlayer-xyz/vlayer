// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ICheatCodes, CHEATCODES} from "../VTest.sol";
import {UnverifiedEmail} from "../../EmailProof.sol";

library EmailTestUtils {
    function preverifyEmail(string memory email) internal view returns (UnverifiedEmail memory) {
        return ICheatCodes(CHEATCODES).preverifyEmail(email);
    }
}
