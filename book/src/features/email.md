# Email

## Email Significance
Many online services, from social media platforms to e-commerce sites, require an email address to create an account. According to recent surveys, more than 80% of businesses consider email to be their primary communication channel, both internally and with customers.

All of this means that our inboxes are full of data that can be leveraged.

## Proof of Email
With vlayer, you can access email content from smart contracts and use it on-chain.

You do this by writing a Solidity smart contract (`Prover`) that has access to the parsed email and returns data to be used on-chain. This allows you to create claims without exposing the full content of an email.

Under the hood, we verify mail server signatures to ensure the authenticity and integrity of the content.

## Email Safety Requirements

Not all emails that are considered valid by email servers will meet the validity requirements for vlayer.
Email servers use various rules based on [DMARC](https://dmarc.org/), [DKIM](https://datatracker.ietf.org/doc/html/rfc6376), and [SPF](https://datatracker.ietf.org/doc/html/rfc7208) to determine if an email is valid.
When creating an Email Proof, only DKIM (DomainKeys Identified Mail) signatures are used to prove the authenticity of an email. Therefore, the following additional preconditions must be met:

- The email must be signed with a DKIM-Signature header.
- The email must be sent from a domain that has a valid DKIM record.
- The email must have exactly one DKIM signature with a [`d`](https://datatracker.ietf.org/doc/html/rfc6376#section-3.5) tag that matches the domain of the `From` header.
- The email must have a signed `From` header containing a single email address.

If the email doesn't have a DKIM signature with matching signer and sender domains, it may indicate that the sender's email server is misconfigured.
Emails from domains hosted on providers like Google Workspaces or Outlook often have a DKIM signature resembling the following:
```
From: Alice <alice.xyz.com>
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed;
   d=xyz-com.***.gappssmtp.com; s=20230601; dara=google.com;
   h=...;
   bh=...;
   b=...
```
Note that the `d` tag domain in this example is `gappssmtp.com`, which is a Google Workspaces domain. The `From` header domain is `xyz.com`. This email will not pass the DKIM validation and fail with the `Error verifying DKIM: signature did not verify` error.

Another potential issue is the use of subdomains.
For example, if the email is sent from `alice@subdomain.example.com` and the `d` tag in the DKIM signature is `example.com`, the email will not be considered valid.
Similarly, if the email is sent from `alice@example.com` and the `d` tag is `subdomain.example.com`, the email will also be invalid.

DKIM validation will fail if the email body has been modified by a proxy server. The body hash included in the DKIM signature ensures the integrity of the email’s content. Any alteration to the body will invalidate the signature.

## DKIM and DNS Notary

The simplified flow of the DKIM signature is:
1. The sender SMTP server has a private and public key pair.
2. The public key is published in DNS as a TXT record under `<selector>._domainkey.<domain>` where:
   - `<selector>` is a unique identifier under `s=` tag in the DKIM-Signature header
   - `_domainkey` is a fixed string
   - `<domain>` is the sender's domain, stored in the `d=` tag in the DKIM-Signature header.
   
3. The email server adds a DKIM-Signature header to the email and sends it.
4. The recipient SMTP server receives the email.
5. SMTP server checks `DKIM-Signature` header, reads d= tag ... stating its singers domain and selector, which gives him notion where to look for the key.
6. The recipient server fetches the public key from DNS and verifies the signature.

The last step becomes tricky: we don't have the access to DNS from the Solidity level.
Instead, we'll have to prove that the DNS record is indeed valid and pass it together with the email to the prover contract.

The `DNS Notary` (aka. Verifiable DNS) service exists for this reason: it uses the [DNS Queries over HTTPS (DoH)](https://datatracker.ietf.org/doc/html/rfc8484) protocol to fetch DNS records from several providers, signs them if they are valid and secure, and returns the signature together with the record.

## Example
Let's say someone wants to prove they are part of a company or organization. One way to do this is to take a screenshot and send it to the verifier. However, this is not very reliable because screenshot images can be easily manipulated, and obviously such an image cannot be verified on-chain.

A better option is to prove that one can send email from their organization domain. Below is a sample `Prover` contract that verifies from which domain an email has been sent.

Below is an example of such proof generation:

```solidity
import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";
import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {RegexLib} from "vlayer-0.1.0/Regex.sol";
import {VerifiedEmail, UnverifiedEmail, EmailProofLib} from "vlayer-0.1.0/EmailProof.sol";

contract EmailDomainProver is Prover {
    using RegexLib for string;
    using Strings for string;
    using EmailProofLib for UnverifiedEmail;

    function main(UnverifiedEmail calldata unverifiedEmail, address targetWallet)
        public
        view
        returns (Proof memory, bytes32, address, string memory)
    {
        VerifiedEmail memory email = unverifiedEmail.verify();
        require(email.subject.equal("Verify me for Email NFT"), "incorrect subject");
        // Extract domain from email address
        string[] memory captures = email.from.capture("^[^@]+@([^@]+)$");
        require(captures.length == 2, "invalid email domain");
        require(bytes(captures[1]).length > 0, "invalid email domain");

        return (proof(), sha256(abi.encodePacked(email.from)), targetWallet, captures[1]);
    }
}
```

It can be convenient to use [Regular Expressions](/features/json-and-regex.md) to validate the content of the email.

Email is passed to the Solidity contract as an `UnverifiedEmail` structure that can be created using the `preverifyEmail` function in the [SDK](../javascript/javascript.md).
`preverifyEmail` should be called with the raw `.eml` file content as an argument ([learn how to get this file](/features/email.html#getting-eml-files)). The email is also required to have [`From` and `DKIM-Signature`](https://datatracker.ietf.org/doc/html/rfc6376) headers.

You can also use the `preverifyEmail` function inside the [Solidity tests](../advanced/tests.md).

```solidity
struct UnverifiedEmail {
  string email;
  string[] dnsRecords;
}
```

First, we verify the integrity of the email with the `verify()` function. Then we have a series of assertions (*regular Solidity `require()`*) that check the email details.

> If one of the string comparisons fails, `require` will revert the execution, and as a result, proof generation will fail.

> 💡 **Try it Now**
>
> To run the above example on your computer, type the following command in your terminal:
>
> ```bash
> vlayer init --template simple-email-proof
> ```
>
> This command will download, create, and initialize a new project with sample email proof contracts.

## Email structure
The `email` structure of type `VerifiedEmail` is a result of the `UnverifiedEmail.verify()` function.
Since the `verify` function actually verifies the passed email, `VerifiedEmail`'s fields can be trusted from this point.

```solidity
struct VerifiedEmail {
  string from;
  string to;
  string subject;
  string body;
}
```
A `VerifiedEmail` consists of the following fields:
- `from` - a string consisting of the sender's email address (*no name is available*);
- `to` - a string consisting of the intended recipient's email address (*no name is available*);
- `subject` - a string with the subject of the email;
- `body` - a string consisting of the entire body of the email.

By inspecting and parsing the email payload elements, we can generate a claim to be used on-chain.

> **Note:** If multiple headers share the same name (for example, two `From:` lines), we always use the last one encountered when parsing or verifying headers (i.e. headers are processed in reverse order). See [RFC 6376 §5.4.2](https://datatracker.ietf.org/doc/html/rfc6376#section-5.4.2) for details.

## Getting `.eml` Files
Obtaining an `.eml` file can be helpful for development purposes, such as testing your own email proofs. Below are instructions for retrieving `.eml` files from common email clients.

### Gmail
1. Open the email you want to save.
2. Click the **three-dot menu** in the top-right corner of the email.
3. Select **Download message**.

![gmail screen instruction](/static/vlayer-eml-1.gif)

### Outlook / Thunderbird
1. Open the email you want to save.
2. Click on the **File** menu.
3. Select **"Save As"**.

## Security Assumptions
Billions of users trust providers to deliver and store their emails. Inboxes often contain critical information, including work-related data, personal files, password recovery links, and more. Email providers also access customer emails for purposes like serving ads. Email proofs can only be as secure as the email itself, and the protocol relies on the trustworthiness of both sending and receiving servers.

### Outgoing Server
The vlayer prover verifies that the message signature matches the public key listed in the DNS records. However, a dishonest outgoing server can forge emails and deceive the prover into generating valid proofs for them. To mitigate this risk, vlayer supports only a limited number of the world's most trusted email providers.

### Preventing Unauthorized Actions
Both outgoing and incoming servers can read emails and use them to create proofs without the permission of the actual mail sender or receiver. This risk also extends to the prover, which accesses the email to generate claims. It is crucial for protocols to utilize email proofs in a manner that prevents the manipulation of smart contracts into performing unauthorized actions, such as sending funds to unintended recipients.

For example, it is advisable to include complete information in the email to ensure correct actions. Opt for emails like: "Send 1 ETH from address X to address Y on Ethereum Mainnet" over partial instructions, like: "Send 1 ETH," where other details come from another source, such as smart contract call parameters. Another approach is to use unique identifiers that unambiguously point to the necessary details.

## `From` header format
We only accept a single `local@domain` address, checked as follows:
* One mailbox: exactly one address, no groups/lists
* `Local` (1–64 chars):
  * May include letters (A–Z, a–z), digits (0–9), and these symbols: `!  #  $  %  &  '  *  +  -  /  =  ?  ^  _  {  |  }  ~`
  * Periods (`.`) can appear between characters, but not at the very start or end, and never two in a row.

* `Domain` (1–255 chars):
  * Made of one or more labels separated by periods, e.g. `example.com` → `example` + `com`
  * Each label can only use letters (`A–Z`, `a–z`), digits (`0–9`), or hyphens (`-`)
  * A label may not begin or end with a hyphen, and labels can’t be empty (no `..`)
