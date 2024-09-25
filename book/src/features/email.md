# Email
<div class="feature-card feature-future">
  <div class="title">
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path fill-rule="evenodd" clip-rule="evenodd" d="M8.05628 8.85927C8.71143 9.48485 8.71143 10.5152 8.05628 11.1408L7.96676 11.2262C6.28748 12.8289 4.99028 14.0665 4.59925 16.6606C4.4609 17.5753 5.24459 18.3334 6.18779 18.3334H10H13.8123C14.7555 18.3334 15.5392 17.5753 15.4008 16.6606C15.0098 14.0665 13.7126 12.8289 12.0333 11.2262L11.9438 11.1408C11.2887 10.5152 11.2887 9.48485 11.9438 8.85927L12.0333 8.77385C13.7126 7.17115 15.0098 5.93355 15.4008 3.33945C15.5392 2.42475 14.7555 1.66669 13.8123 1.66669H10H6.18779C5.24459 1.66669 4.4609 2.42475 4.59925 3.33945C4.99028 5.93355 6.28748 7.17115 7.96676 8.77385L8.05628 8.85927ZM8.62551 7.44458L8.61943 7.4394C7.33973 6.29509 6.38758 5.44324 6.26348 3.3364C6.25574 3.2068 6.31189 3.09044 6.40345 3.01333C6.47466 2.953 6.56783 2.91669 6.66956 2.91669H10H13.3301C13.4502 2.91669 13.5584 2.96735 13.6328 3.04822C13.7024 3.1235 13.7428 3.22502 13.7362 3.3364C13.6121 5.44324 12.6599 6.29509 11.3807 7.4394C11.0421 7.74203 10.6808 8.0652 10.306 8.43744C10.1372 8.60519 9.86251 8.60519 9.69368 8.43744C9.32093 8.06735 8.96168 7.74569 8.62551 7.44458Z" fill="#0052EA"/>
    </svg>
  Future Enhancement
  </div>
  <p>This feature is part of our long-term roadmap. We're excited about its potential and will provide updates as development progresses. </p>
</div>

## Email Significance
Many online services, from social media platforms to e-commerce sites, require an email address to create an account. According to recent surveys, more than 80% of businesses consider email to be their primary communication channel, both internally and with customers. 

All of this means that our inboxes are full of data that can be leveraged.

## Proof of Email
With vlayer, you can access email content from smart contracts and use it on-chain. 

You do this by writing a Solidity smart contract (`Prover`) that has access to the parsed email and returns data to be used on-chain. This allows you to create claims without exposing the full content of an email.

Under the hood, we verify mail server signatures to ensure the authenticity and integrity of the content.

## Example
Let's say someone wants to prove they are a GitHub user. One way to do this is to take a screenshot and send it to the verifier. However, this is not very reliable because screenshot images can be easily manipulated, and obviously such an image cannot be verified on-chain. 

A better option is to prove that GitHub's email servers sent a welcome email. Below is a sample `Prover` contract that verifies that the caller created a GitHub account.

Below is an example of such proof generation:

```solidity
import {Prover} from "vlayer/Prover.sol";
import {UnverifiedEmail, VerifiedEmail, EmailProofLib} from "vlayer/EmailProof.sol";
import {StringUtils} "vlayer/StringUtils.sol";

contract GitHubEmail is Prover {
    using EmailProofLib for UnverifiedEmail;
    using StringUtils for string;

    function main(UnverifiedEmail calldata unverifiedEmail) public view returns (bool) {
        VerifiedEmail memory email = unverifiedEmail.verify();

        require(email.subject.equal("Welcome to GitHub"), "Incorrect subject");
        require(email.from.equal("notifications@github.com"), "Incorrect sender");
        require(email.to.equal("john.prover@gmail.com"), "Incorrect recipient");

        return true;
    }
}
```

Email is passed to the Solidity contract as an `UnverifiedEmail` structure that can be created using the `enrichEmail` function in the [SDK](../javascript/javascript.md).
`enrichEmail` should be called with the raw `.eml` file content as an argument. The email is also required to have "From" and ["DKIM-Signature"](https://datatracker.ietf.org/doc/html/rfc6376) headers.

```solidity
// Note: more fields will be added soon
struct UnverifiedEmail {
  string mime;
  string[] dnsRecords;
}
```

First, we verify the integrity of the email with the `verify()` function. Then we have a series of assertions (*regular Solidity `require()`*) that check the email details. 

String comparison is handled by our `StringUtils` library (*described in more [details below](/features/email.html#stringutils)*). Date values are formatted in the [Unix time](https://en.wikipedia.org/wiki/Unix_time) notation, which allows them to be compared as integers.

> If one of the string comparisons fails, require will revert the execution, and as a result, proof generation will fail.

> 💡 **Try it Now**
> 
> To run the above example on your computer, type the following command in your terminal:
> 
> ```bash
> vlayer init --template email_proof
> ```
> 
> This command will download create and initialise a new project with sample email proof contracts.

## Email structure
The `email` structure of type `VerifiedEmail` is injected into the `Prover` and can be used in a `main()` function.

```solidity
struct VerifiedEmail {
  string subject;
  string body;
  string from;
  string to;
}
```
An `VerifiedEmail` consists of the following fields
- `subject` - a string with the subject of the email
- `body` - a string consisting of the entire body of the email
- `from` - a string consisting of the sender's email address (*no name is available*) 
- `to` - a string consisting of the intended recipient's email address (*no name is available*)

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
import {Prover} from "vlayer/Prover.sol";
import {VerifiedEmail, UnverifiedEmail, EmailProofLib} from "vlayer/EmailProof.sol";
import {StringUtils} from "vlayer/StringUtils.sol"

contract RecoveryEmail is Prover {
    using StringUtils for string;
    using EmailProofLib for UnverifiedEmail;

    function main(address multisigAddr, UnverifiedEmail calldata unverifiedEmail) public returns (address, bytes32, address) {     
      VerifiedEmail memory email = unverifiedEmail.verify()
 
      address lostWallet = parseSubject(email.subject);
      address newAddress = parseBody(email.body);
      bytes32 emailAddrHash = getEmailAddressHash(email.from, multisigAddr, lostWallet);
      
      return (lostWallet, emailAddrHash, newAddress); 
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

    function getEmailAddressHash(string calldata emailAddr, address multisig, address owner) 
      internal 
      returns (bytes32) 
    {
      MultiSigWallet wallet = MultiSigWallet(multisig);

      bytes32 memory recoveryMailHash = wallet.recoveryEmail(owner);
      bytes32 emailAddrHash = keccak256(abi.encodePacked(emailAddr);

      require(recoveryMailHash == emailAddrHash, "Recovery email mismatch");

      return emailAddrHash;
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
* `getEmailAddressHash` compares the email address associated with the wallet with the one received.
  * `recoveryMailHash == emailAddrHash` check if correct email was used for recovery 

On successful execution, proof of computation is returned. It also returns the recovered wallet address, the email address hash, the new wallet address, and the email timestamp as public input.

## Verifier 

Now we are ready to use the proof and results from the previous step for on-chain verification. Valid proof allows us to restore access to `MultiSigWallet` by adding new address to authorized list in smart contract. 

Below is a sample implementation of this:

```solidity 
import { Verifier } from "vlayer/Verifier.sol";

import { RecoveryEmail } from "RecoveryEmail.sol";

address constant PROVER_ADDR = address(0xd7141F4954c0B082b184542B8b3Bd00Dc58F5E05);
bytes4 constant  PROVER_FUNC_SELECTOR = RecoveryEmail.main.selector;

contract MultiSigWallet is Verifier  {
    mapping (address => bool) public owners;
    mapping (address => bytes32) ownerToEmailHash;

    function recovery(
      Proof _p, 
      address lostWallet, 
      bytes32 emailAddrHash, 
      address newOwner, 
    ) 
      public 
      onlyVerified(PROVER_ADDR, PROVER_FUNC_SELECTOR) 
      returns (address) 
    {  
      require(
        ownerToEmailHash[lostWallet] == emailAddrHash, 
        "Recovery email mismatch"
      );

      require(owners[lostWallet]; "Not an owner");
      
      owners[lostWallet] = false;
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
  * `ownerToEmailHash[lostWallet] == emailAddrHash` make sure recovery email address matches the one that was set up previously in the wallet
  * `owners[newOwner] = true` sets up a new wallet to be authorized to use `MultiSigWallet`.


And voilà, we just successfully used email in the context of an on-chain smart contract. 

> Keep in mind that this is a simplified version of a real MultiSig wallet, demonstrating how an email recovery function could operate.


## Security Assumptions
Billions of users trust providers to deliver and store their emails. Inboxes often contain critical information, including work-related data, personal files, password recovery links, and more. Email providers also access customer emails for purposes like serving ads. Email proofs can only be as secure as the email itself, and the protocol relies on the trustworthiness of both sending and receiving servers.

### Outgoing Server
The vlayer prover verifies that the message signature matches the public key listed in the DNS records. However, a dishonest outgoing server can forge emails and deceive the prover into generating valid proofs for them. To mitigate this risk, vlayers support only a limited number of the world's most trusted email providers.

### Preventing Unauthorized Actions
Both outgoing and incoming servers can read emails and use them to create proofs without the permission of the actual mail sender or receiver. This risk also extends to the prover, which accesses the email to generate claims. It is crucial for protocols to utilize email proofs in a manner that prevents the manipulation of smart contracts into performing unauthorized actions, such as sending funds to unintended recipients.

For example, it is advisable to include complete information in the email to ensure correct actions. Opt for emails like: "Send 1 ETH from address X to address Y on Ethereum Mainnet" over partial instructions, like: "Send 1 ETH," where other details come from another source, such as smart contract call parameters. Another approach is to use unique identifiers that unambiguously point to the necessary details.