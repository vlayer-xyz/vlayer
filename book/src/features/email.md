# Email
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

## Email Significance
Many online services, from social media platforms to e-commerce sites, require an email address to create an account. According to recent surveys, more than 80% of businesses consider email to be their primary communication channel, both internally and with customers. 

All of this means that our inboxes are full of data that can be leveraged.

## Proof of Email
With vlayer, you can access email content from smart contracts and use it on-chain. 

You do this by writing a Solidity smart contract (`Prover`) that has access to the parsed email and returns data to be used on-chain. This allows you to create claims without exposing the full content of an email.

Under the hood, we verify mail server signatures to ensure the authenticity and integrity of the content.

## Example
Let's say someone wants to prove they are part of company or organization. One way to do this is to take a screenshot and send it to the verifier. However, this is not very reliable because screenshot images can be easily manipulated, and obviously such an image cannot be verified on-chain. 

A better option is to prove that one can send email from it's organization domain. Below is a sample `Prover` contract that verifies that the sender sent email from a specific domain.

Below is an example of such proof generation:

```solidity
import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";
import {Prover} from "vlayer-0.1.0/src/Prover.sol";
import {VerifiedEmail, UnverifiedEmail, EmailProofLib} from "vlayer-0.1.0/src/EmailProof.sol";
import {EmailStrings} from "./EmailStrings.sol";

contract EmailDomainProver is Prover {
    using Strings for string;
    using EmailStrings for string;
    using EmailProofLib for UnverifiedEmail;

    string targetDomain;

    constructor(string memory _targetDomain) {
        targetDomain = _targetDomain;
    }

    function main(UnverifiedEmail calldata unverifiedEmail, address targetWallet)
        public
        view
        returns (Proof, bytes32, address)
    {
        VerifiedEmail memory email = unverifiedEmail.verify();

        require(email.from.contains(targetDomain), "incorrect sender domain");
        require(email.subject.equal("Verify me for company NFT"), "incorrect subject");

        return (proof(), sha256(abi.encodePacked(email.from)), targetWallet);
    }
}
```

It can be convenient to use [Regular Expressions](./regex-and-json.md) to validate the content of the email.

Email is passed to the Solidity contract as an `UnverifiedEmail` structure that can be created using the `preverifyEmail` function in the [SDK](../javascript/javascript.md).
`preverifyEmail` should be called with the raw `.eml` file content as an argument. The email is also required to have "From" and ["DKIM-Signature"](https://datatracker.ietf.org/doc/html/rfc6376) headers.

```solidity
// Note: more fields will be added soon
struct UnverifiedEmail {
  string email;
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
> vlayer init --template email-proof
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