# Email

## Significance of the mail
Many online services, from social media platforms to e-commerce sites, require an email address to create an account. According to recent surveys, more than 80% of businesses consider email to be their primary communication channel, both internally and with customers. 

All of this means that our inboxes are full of data that can be leveraged.

## Proof of Email
With vlayer, you can access email data from smart contracts and use it on-chain. Email authenticity is automatically proven under the hood. In addition, you can settle claims on-chain without exposing the details or private content of an email.

## Example
Let's say someone wants to prove they've been a Github user since 2020. One way to do this is to take a screenshot and send it to the verifier. However, this is not very reliable as screenshot images can be easily manipulated. The second idea could be to use the Github API, but let's assume that it is not available at the moment.

Another option is to prove that Github mail servers sent a welcome email on a certain date. Below is an example `Prover` contract that verifies that the caller (`msg.sender`) created a Github account before 2020.

```solidity
contract GithubEmail is Prover {
    function main() public returns (bool) {      
      require(mail.subject.equal("Welcome to Github"), "incorrect subject")
      require(mail.from.equal("notifications@github.com"), "incorrect sender")
      require(mail.to[0].compare("john.prover@gmail.com"), "wrong recipient")
      
      // Wed Jan 01 2020 00:00:00 GMT+0100
      require(mail.received_at < 1577833200, "email received after 2020") 

      return true;
    }
}
```
The `mail` structure is automatically injected into the contract context of the mail prover by the vlayer. Then we have a series of assertions (regular Solidity `require()`) that check the mail details. 

String comparison is handled by our `StringUtils` library (described in more [details below](/features/mail.html#stringutils)). Date values are formatted in the [Unix time](https://en.wikipedia.org/wiki/Unix_time) notation, which allows them to be compared as integer values.

> Comparisons with false results or forged data will abort execution and prevent the generation of a valid proof.

## Mail structure
The `mail` structure of type `Mail` is injected into the `Prover` and can be used in a `main()` function.

```solidity
struct Mail {
  string subject;
  string body;
  string from;
  string[] to;
  uint received_at;
}
```
A `Mail` consists of the following fields
- `subject` - a string with the subject of the mail
- `body` - a string consisting of the entire body of the mail
- `from` - a string consisting of the sender's mail address (no name is available) 
- `to` - an array of strings containing the list of emails of the intended recipients (no names available)
- `received_at` - `uint` representing a timestamp of when the email arrived at the destination mail server.

By inspecting and parsing the email payload elements, we can generate a claim to be used on-chain.

## StringUtils
For convenient manipulation of strings, vlayer provides StringUtils library, which consists of functions like:
* `toAddress` - converts a string to an address if properly formatted, otherwise reverts
* `match` - matches RegExp pattern groups and returns them as a string
* `equal` - checks the contents of two strings for equality. Returns true if both are equal, false otherwise.

## Wallet Recovery Example
Below is another example of a `Prover` smart contract parsing an email. However, this time the use case is a bit more advanced and allows the caller to recover access to a MultiSig wallet (a smart contract that allows multiple wallets to authorize transactions).  

The following implementation assumes that the recovery mail is in a predefined format and it extracts data required to recover access to a MultiSig wallet. 

To change the authorized account (recovery procedure), the user just needs to send the following email: 

```
Date: 02 Jul 24 14:52:18+0300 GMT
From: john.prover@example.com
To: <any email we trust>
Subject: Wallet recovery of {old account address}
Body: New wallet address {new account address}
```
Then such email content is extracted using the vlayer SDK (browser extension or local application) and passed to the `Prover` contract that parses it off-chain:

```solidity
contract RecoveryEmail is Prover {
    using StringUtils for string;

    address MULTISIG_ADDR = 0xfcF784b9525D2cbfdF77AbBE61e43b082369f17E;
    MultiSigWallet wallet = MultiSigWallet(MULTISIG_ADDR);

    function main() public returns (string, string, address) {      
      string[] subjectMatches = mail.subject.match(
        "^Wallet recovery of (0x[a-fA-F0-9]{40})$"
      );
      require(subjectMatches.length == 1, "Invalid subject");

      address lostWallet = subjectMatches[0].toAddress();
      string mailHash = keccak256(abi.encodePacked(mail.sender);
      string recoveryMailHash = wallet.recoveryEmail(lostWallet);

      require(
        recoveryMailHash.compare(mailHash),
        "wrong recovery email"
      )

      string[] bodyMatches = mail.body.match(
        "^New wallet address: (0x[a-fA-F0-9]{40})$"
      );
      require(bodyMatches.length == 1, "Invalid body");
      address newAddress = newbodyMatches[0].toAddress();
      
      return (lostWallet, mailHash, newAddress, mail.received_at); 
    }
}
```

What happens step by step in the above snippet? 
* `RecoveryEmail` inherits from `Prover` to have special powers for off-chain proving. 
* `MULTISIG_ADDR` stores the hardcoded address of the Multisig Wallet smart contract. 
* `mail.subject.match` returns strings matching the regular expression for the subject, which must contain the correct wallet address to be recovered.
* The `subjectMatches.length == 1` condition ensures that the subject is not malformed.
* `recoveryMailHash.compare(mailHash)` check if correct email was used for recovery 
* `mail.body.match` get new wallet address from body

On successful execution, proof of computation is returned. It also returns the recovered wallet address, the email address hash, the new wallet address, and the email timestamp as public input.

Now we can use the proof and public inputs for on-chain verification. 




