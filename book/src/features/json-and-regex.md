# JSON Parsing and Regular Expressions

<div class="feature-card feature-in-dev">
  <div class="title">
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M8.57499 3.21665L1.51665 15C1.37113 15.252 1.29413 15.5377 1.29331 15.8288C1.2925 16.1198 1.3679 16.4059 1.51201 16.6588C1.65612 16.9116 1.86392 17.1223 2.11474 17.2699C2.36556 17.4174 2.65065 17.4968 2.94165 17.5H17.0583C17.3493 17.4968 17.6344 17.4174 17.8852 17.2699C18.136 17.1223 18.3439 16.9116 18.488 16.6588C18.6321 16.4059 18.7075 16.1198 18.7067 15.8288C18.7058 15.5377 18.6288 15.252 18.4833 15L11.425 3.21665C11.2764 2.97174 11.0673 2.76925 10.8176 2.62872C10.568 2.48819 10.2864 2.41437 9.99999 2.41437C9.71354 2.41437 9.43193 2.48819 9.18232 2.62872C8.93272 2.76925 8.72355 2.97174 8.57499 3.21665V3.21665Z" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 7.5V10.8333" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 14.1667H10.0083" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
    Actively in Development
  </div>
  <p>Our team is currently working on this feature. If you experience any bugs, please let us know <a href="https://discord.gg/JS6whdessP" target="_blank">on our Discord</a>. We appreciate your patience. </p>
</div>

When dealing with [Web Proofs](/features/web.html), the ability to parse JSON data is essential. Similarly, finding specific strings or patterns in the subject or body of an email is crucial for [Email Proofs](/features/email.html). 

To support these needs, we provide helpers for parsing text using [regular expressions](https://en.wikipedia.org/wiki/Regular_expression) and extracting data from JSON directly within vlayer `Prover` contracts.

## JSON Parsing

We provide three functions to extract data from JSON based on the field type:
- `jsonGetInt`: Extracts an integer value and returns `int256`.
- `jsonGetBool`: Extracts a boolean value and returns `bool`.
- `jsonGetString`: Extracts a string value and returns `string memory`.

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

We provide a set of functions to match a regular expression against a string.
- `matches` checks if a string matches a regular expression and returns `true` if a match is found;
- `find` - as an addition to `matches`, it returns a tuple `(bool, Match)`, where the second param indicates the index of the string where the matched substring starts, in case if many substrings match, the first one is returned;
- `findAll` - `matches` and `find` functions return on the first match, whereas this function returns a list of tuples pointing to all starting positions of the matched substrings; 
- `captures` -  TODO
- `capturesAll` - TODO


```solidity

struct Match {
  uint256 beginOffset;
  uint256 endOffset; // first character not included in the match
  string value;
}

struct CatureResult {
  uint256 beginOffset;
  uint256 endOffset; // first character not included in the match
  string value;
  Capture[] captures;
}

struct Capture {
  string name;
  uint256 beginOffset;
  uint256 endOffset; // first character not included in the match
  string value;
}

interface RegexLib {


  function match(string memory text, string memory regex) returns (bool);
  function find(string memory text, string memory regex) returns (bool, Match memory) ;

  function captures(string memory text, string memory regex) returns (bool, CaptureResult memory);

  function findAll(string memory text, string memory regex) returns (bool, Match[] memory) ;
  function capturesAll(string memory text, string memory regex) returns (bool, CaptureResult[] memory);
  
}

interface CaptureResultLib {

  // search capture by its name
  function get(CaptureResult memory capture, string memory name) returns (Capture memory); 

}

```

```solidity
import {Prover} from "vlayer/Prover.sol";
import {RegexLib} from "vlayer/Regex.sol";

contract RegexMatchProof is Prover {
    using RegexLib for string;

    function main(string calldata text) public returns (Proof memory, string memory) {
        // The regex pattern is passed as a string
        require(text.matches("^[a-zA-Z0-9]*$"), "text must be alphanumeric only");

        // Return proof and provided text if it matches the pattern
        return (proof(), text);
    }
}
```

### Which function to use?
It's worth noting that regular expressions can become computation intensive, which may significantly increase both execution and proving time.
It's crucial to only ask for what is actually needed, which is the reasoning for providing not just a one, generic regex function, but a whole set of them; 
so that one can decide which function suits the use-case.

### Algorithm complexity (aka. Big-O notation)
Given:
- `m` - linear length of the regular expression, wher linear length is the length of the regex after all repetitions has been expanded ex. `a{5}` -> `aaaaa`;
- `n` - length of the string matched against

Then the complexity of each of the operation is as following:
- `match`, `find`, `find_captures` - bound by `O(m*n)`;
- `find_all`, `captures_all` - are bound by `O(m*n^2)`, since in worst case scenarios we have to make the whole computation `n` times, to find all occurences. 

### Limits
Upper limits are defined as below:
- `m` is limited to 100;
- `n` is limited to 5 242 880 characters (~5Mb);
- `n*m` is limited to 5 242 880.

### Syntax
Underneath, vlayer uses [`regex`](https://docs.rs/regex/latest/regex) rust crate, so syntax if fully compatible with the library.
Regex playground is available [here](https://rustexp.lpil.uk).
