// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer-0.1.0/WebProof.sol";

contract WebProofProver is Prover {
    using Strings for string;
    using WebProofLib for WebProof;
    using WebLib for Web;

    string dataUrl = "https://www.accountable.capital:10443/binance";

    function main(WebProof calldata webProof)
        public
        view
        returns (Proof memory, string[] memory, int[] memory)
    {
        Web memory web = webProof.verify(dataUrl);

        uint256 len = web.jsonGetArrayLength("");
        require(len == 4, "Expected 4 assets");

        string[] memory assetNames = new string[](len);
        int[] memory assetValues = new int[](len);
    

        for (uint i = 0; i < len; i++) {
            string memory indexStr = Strings.toString(i);
            string memory assetKey = string(abi.encodePacked("[", indexStr, "].asset"));
            string memory valueKey = string(abi.encodePacked("[", indexStr, "].free"));

            string memory assetName = web.jsonGetString(assetKey);
            int assetValue = parseDecimal(web.jsonGetString(valueKey), 8);

            assetNames[i] = assetName;
            assetValues[i] = assetValue;
        }

        return (proof(), assetNames, assetValues);
    }

    function parseDecimal(string memory input, uint decimalPlaces) internal pure returns (int) {
        bytes memory inputBytes = bytes(input);
        if (inputBytes.length == 0) {
            return 0;
        }

        int result = 0;
        bool isNegative = false;
        uint i = 0;

        // Check for negative sign
        if (inputBytes[0] == '-') {
            isNegative = true;
            i = 1;
        }

        bool decimalFound = false;
        uint decimalIndex = 0;
        for (; i < inputBytes.length; i++) {
            if (inputBytes[i] == '.') {
                decimalFound = true;
                decimalIndex = i;
                break;
            }
        }

        if (!decimalFound) {
            decimalIndex = inputBytes.length;
        }

        uint factor = 10 ** decimalPlaces;
        uint integerPart = 0;
        uint fractionalPart = 0;

        for (i = isNegative ? 1 : 0; i < decimalIndex; i++) {
            require(inputBytes[i] >= '0' && inputBytes[i] <= '9', "Invalid character in integer part");
            integerPart = integerPart * 10 + (uint8(inputBytes[i]) - 48);
        }

        for (i = decimalIndex + 1; i < inputBytes.length && i < decimalIndex + 1 + decimalPlaces; i++) {
            require(inputBytes[i] >= '0' && inputBytes[i] <= '9', "Invalid character in fractional part");
            fractionalPart = fractionalPart * 10 + (uint8(inputBytes[i]) - 48);
        }

        uint fractionalLength = inputBytes.length > decimalIndex + 1 ? inputBytes.length - (decimalIndex + 1) : 0;
        if (fractionalLength < decimalPlaces) {
            fractionalPart *= 10 ** (decimalPlaces - fractionalLength);
        }

        result = int(integerPart * factor + fractionalPart);
        return isNegative ? -result : result;
    }
}
