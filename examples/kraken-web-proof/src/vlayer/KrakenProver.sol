// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer-0.1.0/WebProof.sol";

contract KrakenProver is Prover {
    using WebProofLib for WebProof;
    using WebLib for Web;

    string public constant DATA_URL = "https://api.kraken.com/0/public/Ticker?pair=ETHUSD";

    function main(WebProof calldata webProof) public view returns (Proof memory, string memory) {
        Web memory web = webProof.verify(DATA_URL);

        string memory avgPrice = web.jsonGetString("result.XETHZUSD.p[0]");

        return (proof(), avgPrice);
    }
}
