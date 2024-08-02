# Email

## Email Significance
Many online services, from social media platforms to e-commerce sites, require an email address to create an account. According to recent surveys, more than 80% of businesses consider email to be their primary communication channel, both internally and with customers. 

All of this means that our inboxes are full of data that can be leveraged.

## Proof of Email
With vlayer, you can access email content from smart contracts and use it on-chain. 

You do this by writing a Solidity smart contract (`Prover`) that has access to the parsed email and returns data to be used on-chain. This allows you to create claims without exposing the full content of an email.

Under the hood, we verify mail server signatures to ensure the authenticity and integrity of the content.

## Example
Let's say someone wants to prove they've been a GitHub user since 2020. One way to do this is to take a screenshot and send it to the verifier. However, this is not very reliable because screenshot images can be easily manipulated, and obviously such an image cannot be verified on-chain. 

A better option is to prove that GitHub's email servers sent a welcome email on a certain date. Below is a sample `Prover` contract that verifies that the caller (`msg.sender`) created a GitHub account before 2020.

Below is an example of such proof generation:

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


The `email` structure is automatically injected into the contract context of the email prover by the vlayer. Then we have a series of assertions (*regular Solidity `require()`*) that check the email details. 

String comparison is handled by our `StringUtils` library (*described in more [details below](/features/email.html#stringutils)*). Date values are formatted in the [Unix time](https://en.wikipedia.org/wiki/Unix_time) notation, which allows them to be compared as integers.

> If one of the string comparisons fails, require will revert the execution, and as a result, proof generation will fail.

> 💡 **Try it Now**
> 
> To run the above example on your computer, type the following command in your terminal:
> 
> ```bash
> vlayer init --template email_example
> ```
> 
> This command will download all necessary artifacts to your project.

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
An `Email` consists of the following fields
- `subject` - a string with the subject of the email
- `body` - a string consisting of the entire body of the email
- `from` - a string consisting of the sender's email address (*no name is available*) 
- `to` - an array of strings containing the list of emails of the intended recipients (*no names available*)
- `received_at` - `uint` representing a timestamp of when the email arrived at the destination email server.

By inspecting and parsing the email payload elements, we can generate a claim to be used on-chain.

## StringUtils
For convenient manipulation of strings, vlayer provides `StringUtils` library, which consists of functions like:
* `toAddress` - converts a string to an address if properly formatted, reverts otherwise
* `match` - matches RegExp pattern groups and returns them as a string
* `equal` - checks the contents of two strings for equality. Returns true if both are equal, false otherwise.

## Wallet Recovery Example
Below is another example of a `Prover` smart contract parsing an email. This time, however, the use case is a bit more advanced. It allows the caller to recover access to a MultiSig wallet (*a smart contract that allows multiple wallets to authorize transactions*).  

The following implementation assumes that the recovery email is in a predefined format. It extracts the data needed to restore access to a MultiSig wallet. 

To change the authorized account (*recovery procedure*), the user simply needs to send the email in the following format: 

```
Date: 02 Jul 24 14:52:18+0300 GMT
From: john.prover@example.com
To: <any email with trusted provider>
Subject: Wallet recovery of {old account address}
Body: New wallet address {new account address}
```
Now, we can access the email from the `Prover` contract:

```solidity
contract RecoveryEmail is Prover {
    using StringUtils for string;

    function main(address multisigAddr) public returns (address, string, address, uint) {      
      address lostWallet = parseSubject(email.subject);
      address newAddress = parseBody(email.body);
      string memory emailHash = getEmailHash(email.sender, multisigAddr, lostWallet);
      
      return (lostWallet, emailHash, newAddress, email.received_at); 
    }

    function parseSubject(string calldata subject) internal returns (address) {
      string[] subjectMatches = subject.match(
        "^Wallet recovery of (0x[a-fA-F0-9]{40})$"
      );
      require(subjectMatches.length == 1, "Invalid subject");

      return subjectMatches[0].toAddress();
    }

    function parseBody(string calldata body) internal returns (address) {
      string[] bodyMatches = body.match(
        "^New wallet address: (0x[a-fA-F0-9]{40})$"
      );
      require(bodyMatches.length == 1, "Invalid body");
      
      return newbodyMatches[0].toAddress();
    }    

    function getEmailHash(string calldata email, address multisig, address owner) 
      internal 
      returns (string) 
    {
      MultiSigWallet wallet = MultiSigWallet(multisig);

      string memory recoveryMailHash = wallet.recoveryEmail(owner);
      string memory emailHash = keccak256(abi.encodePacked(email);

      require(recoveryMailHash.equal(emailHash), "wrong recovery email")

      return emailHash;
    }
}
```

What happens step by step in the above snippet? 
* `RecoveryEmail` inherits from `Prover` to obtain super powers of off-chain proving. 
* `main` function takes `multisigAddr` argument to access Multisig Wallet smart contract data. 
* `parseSubject` parses email subject and returns address of lost wallet
  * `email.subject.match` returns strings matching the regular expression for the subject, which must contain the correct wallet address to be recovered.
  * The `subjectMatches.length == 1` condition ensures that the subject is not malformed.
* `parseBody` extracts new owner address 
  * `email.body.match` retrieves new wallet address from the email body
* `getEmailHash` compares the email associated with the wallet with the one received.
  * `recoveryMailHash.equal(emailHash)` check if correct email was used for recovery 

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
      Proof _p, 
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

* First, note we need to let know `Verifier`, which `Prover` contract to verify:
  * The `PROVER_ADDR` constant holds the address of the `Prover` contract that generated the proof. 
  * The `PROVER_FUNC_SELECTOR` constant holds the selector for the `Prover.main()` function. 
  * `MultiSigWallet` inherits from `Verifier`, so we can call the `onlyVerified` modifier that makes sure the proof is correct or it will revert otherwise.

* Next, we add two fields that an example MultiSig might use:
  * The `owners` mapping holds addresses that can use `MultiSigWallet`.
  * The `ownerToEmailHash` mapping holds hashes of email addresses associated with owners. 

* Finally, we need a logic that will perform all verifications and do actual recovery: 
  * The `recovery()` function takes follwoing arguments:`proof` and returned values generated by `Prover.main()`.
  * `onlyVerified(PROVER_ADDR, PROVER_FUNC_SELECTOR)` validates execution of Prover and correctness of arguments. If the proof is invalid or arguments don't match returned values it will revert. 
  * You don't need to pass `proof` as an argument to `onlyVerified` because it is automatically extracted from `msg.data`.
  * `ownerToEmailHash[lostWallet] == emailHash` make sure recovery email address matches the one that was set up previously in the wallet
  * `(block.timestamp - recoveryDate) <= 86400` call makes sure recovery email isn't older than 24h, otherwise reverts
  * `owners[newOwner] = true` sets up a new wallet to be authorized to use `MultiSigWallet`.


And voilà, we just successfully used email in the context of an on-chain smart contract. 

> Keep in mind that this is a simplified version of a real MultiSig wallet, demonstrating how an email recovery function could operate.


## Security Assumptions
Billions of users trust providers to deliver and store their emails. Inboxes often contain critical information, including work-related data, personal files, password recovery links, and more. Email providers also access customer emails for purposes like serving ads. Email proofs can only be as secure as the email itself, and the protocol relies on the trustworthiness of both sending and receiving servers.

### Outgoing Server
The vlayer prover verifies that the message signature matches the public key listed in the DNS records. However, a dishonest outgoing server can forge emails and deceive the prover into generating valid proofs for them. To mitigate this risk, vlayers support only a limited number of the world's most trusted email providers.

### Preventing Unauthorized Actions
Both outgoing and incoming servers can read emails and use them to create proofs without the permission of the actual mail sender or receiver. This risk also extends to the prover, which accesses the email to generate claims. It is crucial for protocols to utilize email proofs in a manner that prevents the manipulation of smart contracts into performing unauthorized actions, such as sending funds to unintended recipients.

For example, it is advisable to include complete information in the email to ensure correct actions. Prefer emails like: "Send 1 ETH from address X to address Y on Ethereum Mainnet" over partial instructions like: "Send 1 ETH," where other details come from another source, such as smart contract call parameters. Another approach is to use unique identifiers that unambiguously point to the necessary details.