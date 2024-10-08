# Text Parsing

<div class="feature-card feature-future">
  <div class="title">
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path fill-rule="evenodd" clip-rule="evenodd" d="M8.05628 8.85927C8.71143 9.48485 8.71143 10.5152 8.05628 11.1408L7.96676 11.2262C6.28748 12.8289 4.99028 14.0665 4.59925 16.6606C4.4609 17.5753 5.24459 18.3334 6.18779 18.3334H10H13.8123C14.7555 18.3334 15.5392 17.5753 15.4008 16.6606C15.0098 14.0665 13.7126 12.8289 12.0333 11.2262L11.9438 11.1408C11.2887 10.5152 11.2887 9.48485 11.9438 8.85927L12.0333 8.77385C13.7126 7.17115 15.0098 5.93355 15.4008 3.33945C15.5392 2.42475 14.7555 1.66669 13.8123 1.66669H10H6.18779C5.24459 1.66669 4.4609 2.42475 4.59925 3.33945C4.99028 5.93355 6.28748 7.17115 7.96676 8.77385L8.05628 8.85927ZM8.62551 7.44458L8.61943 7.4394C7.33973 6.29509 6.38758 5.44324 6.26348 3.3364C6.25574 3.2068 6.31189 3.09044 6.40345 3.01333C6.47466 2.953 6.56783 2.91669 6.66956 2.91669H10H13.3301C13.4502 2.91669 13.5584 2.96735 13.6328 3.04822C13.7024 3.1235 13.7428 3.22502 13.7362 3.3364C13.6121 5.44324 12.6599 6.29509 11.3807 7.4394C11.0421 7.74203 10.6808 8.0652 10.306 8.43744C10.1372 8.60519 9.86251 8.60519 9.69368 8.43744C9.32093 8.06735 8.96168 7.74569 8.62551 7.44458Z" fill="#0052EA"/>
    </svg>
  Future Enhancement
  </div>
  <p>This feature is part of our long-term roadmap. We're excited about its potential and will provide updates as development progresses. </p>
</div>

Finding patterns in text can be extremely useful while making proofs. When verifying Web Proofs, it is also essential to be able to parse JSON data.
To make it possible, we provide tools to parse text using regular expressions and JSON data directly in Solidity contracts.

## JSON Parsing

We currently provide 3 functions to extract data from JSON depending on the type of field:

- `jsonGetInt` - extracts an integer value from a JSON field, returns `int256`.
- `jsonGetBool` - extracts a boolean value from a JSON field, returns `bool`.
- `jsonGetString` - extracts a string value from a JSON field, returns `string memory`.

```solidity
import {Prover} from "vlayer/Prover.sol";
import {WebLib} from "vlayer/WebProof.sol";

contract JSONContainsFieldProof is Prover {
    using WebLib for string;

    function main(string calldata json) public returns (Proof memory, string memory) {
        require(json.jsonGetInt("deep.nested.field") == 42, "deep nested field is not 42");
        
        // If we return the provided JSON back, we will be able to pass it to verifier
        // Together with a proof that it contains the field
        return (proof(), json);
    }
}
```

Paths to the fields are separated by dots. The above code will extract the value of the field `deep.nested.field` from the JSON-formatted string, compare it with `42`.
Functions will revert in case the field under the path does not exist or the value is not of the expected type.

Accessing fields inside an array is not currently supported.

## Regular Expressions

[Regular expressions](https://en.wikipedia.org/wiki/Regular_expression) are a powerful tool for finding patterns in text.
We provide a function to match a regular expression against a string:

- `matches` - matches a regular expression against a string, returns `true` only if a match exists.

```solidity
import {Prover} from "vlayer/Prover.sol";
import {RegexLib} from "vlayer/Regex.sol";

contract RegexMatchProof is Prover {
    using RegexLib for string;

    function main(string calldata text) public returns (Proof memory, string memory) {
        // The regex is passed as a normal string
        require(text.matches("^[a-zA-Z0-9]*$"), "text contains invalid characters");
        
        // Return text back to prove it contains only alphanumeric characters
        return (proof(), text);
    }
}
```

 