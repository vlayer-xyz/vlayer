# Email

## Significance of the mail
Many online services, from social media platforms to e-commerce sites, require an email address for account creation. According to recent surveys, over 80% of businesses consider email to be their primary communication channel, both internally and with customers.  

All of this means that our inboxes are full of data that can be leveraged.

## Proof of Email
With vlayer, you can access emails data from smart contracts and utilize it on-chain. The email authenticity is automatically proven under the hood. Moreover, you can generate claims on-chain without revealing detail content of the mail. 

## Mail structure

```solidity
struct {
  string subject;
  string body;
  string from;
  string[] to;
  uint received_at;
}
```

The `mail` structure of type `Mail` is injected into the `Prover` and can be used in a function. A `Mail` consists of the following fields:
- `subject` - a string with the subject of the mail
- `body` - a string consisting of the entire email body
- `from` -  a string consisting of the email address of the sender (no name is available). 
- `to` - an array of strings containing the list of emails of the intended recipients (no names available)
- `received_at` - `uint` representing a timestamp when the email arrived at the destination mail server.

By inspecting and parsing email payload elements, we can generate a claim to be leveraged on-chain.

## StringUtils
For convenient manipulation of strings, vlayer provides `StringUtils` library, which consists of functions like:
* `toAddress` - converts a string to an address if properly formatted, otherwise reverts
* `match` - matches RegExp pattern groups and returns them as a string

## Example
Below is an example of a `Prover` smart contract parsing a mail. It assumes the mail is in predefined format and it extracts data required to recover an access to a multisig wallet.

```solidity
contract RecoveryEmail is Prover {
    using StringUtils for string;

    function main() public returns (string, string, address) {      
      string[] subjectMatches = mail.subject.match(
        "^Wallet recovery of (0x[a-fA-F0-9]{40})$"
      );
      require(subjectMatches.length == 1, "Invalid subject");
      address contractAddress = subjectMatches[0].toAddress();

      string mailHash = keccak256(abi.encodePacked(mail.sender);

      string[] bodyMatches = mail.body.match(
        "^New wallet address: (0x[a-fA-F0-9]{40})$"
      );
      require(bodyMatches.length == 1, "Invalid body");
      address newAddress = newbodyMatches[0].toAddress();
      
      return (contractAddress, mailHash, newAddress, mail.received_at); 
    }
}
```
