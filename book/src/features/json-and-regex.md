# JSON Parsing and Regular Expressions

When dealing with [Web Proofs](/features/web.html), the ability to parse JSON data is essential. Similarly, finding specific strings or patterns in the subject or body of an email is crucial for [Email Proofs](/features/email.html). 

To support these needs, we provide helpers for parsing text using [regular expressions](https://en.wikipedia.org/wiki/Regular_expression) and extracting data from JSON directly within vlayer `Prover` contracts.

## JSON Parsing

We provide four functions to extract data from JSON based on the field type:
- `jsonGetInt`: Extracts an integer value and returns `int256`;
- `jsonGetBool`: Extracts a boolean value and returns `bool`;
- `jsonGetString`: Extracts a string value and returns `string memory`;
- `jsonGetArrayLength`: Returns length of an array under provided `jsonPath`, returns `uint256`. 

```solidity
import {Prover} from "vlayer/Prover.sol";
import {Web, WebLib} from "vlayer/WebProof.sol";

contract JSONContainsFieldProof is Prover {
    using WebLib for Web;

    function main(Web memory web) public returns (Proof memory, string memory) {
        require(web.jsonGetInt("deep.nested.field") == 42, "deep nested field is not 42");
        
        // If we return the provided JSON back, we will be able to pass it to verifier
        // Together with a proof that it contains the field
        return (proof(), web.body);
    }
}
```

In the example above, the function extracts the value of the field `deep.nested.field` from the JSON string below and checks if it equals `42`.

```json
{
  "deep": {
    "nested": {
      "field": 42
    }
  }
}
```

The functions will revert if the field does not exist or if the value is of the wrong type. 

Currently, accessing fields inside arrays is not supported.

## Regular Expressions
Regular expressions are a powerful tool for finding patterns in text.

We provide functions to match and capture a substring using regular expressions:
- `matches` checks if a string matches a regular expression and returns `true` if a match is found;
- `capture` checks if a string matched a regular expression and returns an array of strings. First string is the whole matched text, followed by the captures.

## Regex size optimization
Internally, the regular expression is compiled into a [DFA](https://en.wikipedia.org/wiki/Deterministic_finite_automaton).
The size of the DFA is determined by the regular expression itself, and it can get quite large even for seemingly simple patterns.
It's important to remember that the DFA size corresponds to the cycles used in the ZK proof computation, and therefore it is important to keep it as small as possible.
We have a hard limit for a DFA size which should be enough for most use cases. 
For example the regex `"\w"` includes all letters including the ones from unicode and as a result will be over 100x larger than a simple `"[a-zA-Z0-9]"` pattern.
In general, to bring the compiled regular expression size down, it is recommended to use more specific patterns.


```solidity
import {Prover} from "vlayer/Prover.sol";
import {RegexLib} from "vlayer/Regex.sol";

contract RegexMatchProof is Prover {
    using RegexLib for string;

    function main(string calldata text, string calldata hello_world) public returns (Proof memory, string memory) {
        // The regex pattern is passed as a string
        require(text.matches("^[a-zA-Z0-9]*$"), "text must be alphanumeric only");

        // Example for "hello world" string 
        string[] memory captures = hello_world.capture("^hello(,)? (world)$");
        assertEq(captures.length, 3);
        assertEq(captures[0], "hello world");
        assertEq(captures[1], "");
        assertEq(captures[2], "world");

        // Return proof and provided text if it matches the pattern
        return (proof(), text);
    }
}
```
