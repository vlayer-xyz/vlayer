// SPDX-License-Identifier: UNLICENSED
import {SimpleTravelProver} from "./SimpleTravelProver.sol";
import {Verifier} from "vlayer/Verifier.sol";

contract SimpleTravel is Verifier {
    address public prover;

    constructor(address _prover) {
        prover = _prover;
    }

    function verify()
        public
        onlyVerified(prover, SimpleTravelProver.aroundTheWorld.selector)
    {}
}
