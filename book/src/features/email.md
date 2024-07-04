# Email

## Significance of the email
Many online services, from social media platforms to e-commerce sites, require an email address to create an account. According to recent surveys, more than 80% of businesses consider email to be their primary communication channel, both internally and with customers. 

All of this means that our inboxes are full of data that can be leveraged.

## Proof of Email
With vlayer, you can access email data from smart contracts and use it on-chain. Email authenticity is automatically proven under the hood. In addition, you can settle claims on-chain without exposing the details or private content of an email.

## Example
Let's say someone wants to prove they've been a GitHub user since 2020. One way to do this is to take a screenshot and send it to the verifier. However, this is not very reliable because screenshot images can be easily manipulated, and obviously such an image cannot be verified on-chain. 

A better option is to prove that GitHub's email servers sent a welcome email on a certain date. Below is a sample `Prover` contract that verifies that the caller (`msg.sender`) created a GitHub account before 2020.

Below is an example of such proof genration:

```solidity
contract GitHubEmail is Prover {
    function main() public returns (bool) {      
      require(email.subject.equal("Welcome to GitHub"), "Incorrect subject")
      require(email.from.equal("notifications@github.com"), "Incorrect sender")
      require(email.to[0].equal("john.prover@gmail.com"), "Incorrect recipient")
      
      // Wed Jan 01 2020 00:00:00 GMT+0100
      require(email.received_at < 1577833200, "Email received after 2020") 

      return true;
    }
}
```
The `email` structure is automatically injected into the contract context of the email prover by the vlayer. Then we have a series of assertions (regular Solidity `require()`) that check the email details. 

String comparison is handled by our `StringUtils` library (described in more [details below](/features/email.html#stringutils)). Date values are formatted in the [Unix time](https://en.wikipedia.org/wiki/Unix_time) notation, which allows them to be compared as integers.

> If one of the string comparisons fails, require will revert the execution, and as a result, proof generation will fail.

## Email structure
The `email` structure of type `Email` is injected into the `Prover` and can be used in a `main()` function.

```solidity
struct Email {
  string subject;
  string body;
  string from;
  string[] to;
  uint received_at;
}
```
A `Email` consists of the following fields
- `subject` - a string with the subject of the email
- `body` - a string consisting of the entire body of the email
- `from` - a string consisting of the sender's email address (no name is available) 
- `to` - an array of strings containing the list of emails of the intended recipients (no names available)
- `received_at` - `uint` representing a timestamp of when the email arrived at the destination email server.

By inspecting and parsing the email payload elements, we can generate a claim to be used on-chain.

## StringUtils
For convenient manipulation of strings, vlayer provides StringUtils library, which consists of functions like:
* `toAddress` - converts a string to an address if properly formatted, reverts otherwise
* `match` - matches RegExp pattern groups and returns them as a string
* `equal` - checks the contents of two strings for equality. Returns true if both are equal, false otherwise.

## Wallet Recovery Example
Below is another example of a `Prover` smart contract parsing an email. However, this time the use case is a bit more advanced and allows the caller to recover access to a MultiSig wallet (a smart contract that allows multiple wallets to authorize transactions).  

The following implementation assumes that the recovery email is in a predefined format and it extracts data required to recover access to a MultiSig wallet. 

To change the authorized account (recovery procedure), the user just needs to send the email in the following format: 

```
Date: 02 Jul 24 14:52:18+0300 GMT
From: john.prover@example.com
To: <any email with trusted provider>
Subject: Wallet recovery of {old account address}
Body: New wallet address {new account address}
```
Now, we you can access the email from the `Prover` contract:

```solidity
contract RecoveryEmail is Prover {
    using StringUtils for string;

    function main(address multisigAddr) public returns (string, string, address) {      
      string[] subjectMatches = email.subject.match(
        "^Wallet recovery of (0x[a-fA-F0-9]{40})$"
      );
      require(subjectMatches.length == 1, "Invalid subject");

      address lostWallet = subjectMatches[0].toAddress();
      string emailHash = keccak256(abi.encodePacked(email.sender);
      MultiSigWallet wallet = MultiSigWallet(multisigAddr);
      string recoveryMailHash = wallet.recoveryEmail(lostWallet);

      require(
        recoveryMailHash.equal(emailHash),
        "wrong recovery email"
      )

      string[] bodyMatches = email.body.match(
        "^New wallet address: (0x[a-fA-F0-9]{40})$"
      );
      require(bodyMatches.length == 1, "Invalid body");
      address newAddress = newbodyMatches[0].toAddress();
      
      return (lostWallet, emailHash, newAddress, email.received_at); 
    }
}
```

What happens step by step in the above snippet? 
* `RecoveryEmail` inherits from `Prover` to obtain super powers of off-chain proving. 
* `main()` function takes `multisigAddr` argument to access Multisig Wallet smart contract data. 
* `email.subject.match` returns strings matching the regular expression for the subject, which must contain the correct wallet address to be recovered.
* The `subjectMatches.length == 1` condition ensures that the subject is not malformed.
* `recoveryMailHash.equal(emailHash)` check if correct email was used for recovery 
* `email.body.match` retrieves new wallet address from the email body

On successful execution, proof of computation is returned. It also returns the recovered wallet address, the email address hash, the new wallet address, and the email timestamp as public input.

## Verifier 

Now we are ready to use the proof and results from the previous step for on-chain verification. Valid proof allows us to restore access to `MultiSigWallet` by adding new address to authorized list in smart contract. 

Below is a sample implementation of this:

```solidity 
import { RecoveryEmail } from "./v/RecoveryEmail.v.sol";

address constant PROVER_ADDR = 0xd7141F4954c0B082b184542B8b3Bd00Dc58F5E05;
bytes4 constant  PROVER_FUNC_SELECTOR = RecoveryEmail.main.selector;

contract MultiSigWallet is Verifier  {    
    mapping (address => bool) public owners;
    mapping (address => string) ownerToEmailHash;

    function recovery(
      Proof proof, 
      address lostWallet, 
      string emailHash, 
      address newOwner, 
      uint recoveryDate
    ) 
      public 
      onlyVerified(PROVER_ADDR, PROVER_FUNC_SELECTOR) 
      returns (address) 
    {  
      require(
        ownerToEmailHash[lostWallet] == emailHash, 
        "wrong email given"
      );
      require(
        (block.timestamp - recoveryDate) <= 86400, 
        "email older than 24h"
      );

      owners[newOwner] = true;

      return (newOwner); 
    }
}
```

What exactly happened in the above code? 

* First, note we need to let know Verifier, which Prover contract to verify:
  * The `PROVER_ADDR` constant holds the address of the `Prover` contract that generated the proof. 
  * The `PROVER_FUNC_SELECTOR` constant holds the selector for the `Prover.main()` function. 
  * `MultiSigWallet` inherits from `Verifier`, so we can call the `onlyVerified` modifier that makes sure the proof is correct or it will revert otherwise.

* Next, we add two fields that an example MultiSig might use:
  * The `owners` mapping holds addresses that can use `MultiSigWallet`.
  * The `ownerToEmailHash` mapping holds hashes of email addresses associated with owners. 

* Finally, we need a logic that will perform all verifications and do actual recovery: 
  * The `recovery()` function takes follwoing arguments:`proof` and returned values generated by `Prover.main()`.
  * `onlyVerified(PROVER_ADDR, PROVER_FUNC_SELECTOR)` validates execution of Prover and correctness of arguments. If the proof is invalid or arguments don't match returned values it will revert. 
  * `ownerToEmailHash[lostWallet] == emailHash` make sure recovery email address matches the one that was set up previously in the wallet
  * `(block.timestamp - recoveryDate) <= 86400` call makes sure recovery email isn't older than 24h, otherwise reverts
  * `owners[newOwner] = true` sets up a new wallet to be authorized to use `MultiSigWallet`.


And voilà, we just successfully used email in the context of an on-chain smart contract. 

> Please note this is just a simplification of what a real MultiSig wallet would look like, but it shows how email recovery function could work.

