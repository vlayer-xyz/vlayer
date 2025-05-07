# Redaction

## What is Redaction?

Redaction feature lets you **hide sensitive portions** of an HTTPS transcript from the Prover. Common use cases include removing cookies, authorization headers, or API tokens before generating a Web Proof. At the same time, everything you **leave visible** is still **cryptographically verified** for integrity.

> ⚠️ **Warning!** Unsafe byte-range redaction can introduce ambiguities and vulnerabilities. Strategies to avoid these risks and safely apply redaction are described below.

To learn how to enable and configure redaction using the vlayer SDK, see the [Redaction](../../javascript/web-proofs.md#redaction) section in our JavaScript documentation.

### Currently supported redaction targets:
* URL path
* Request headers
* Response headers

> **Note:** Redaction of the response body is currently supported only in *unsafe* mode, which demands special security concerns. More details will be provided in a future book update.

---

## Security Model

### Why Caution Is Needed?

In [TLSN](https://tlsnotary.org/), the foundation for Web Proofs, redaction is performed over raw byte ranges. This means the transcript is treated as an unstructured byte stream, without awareness of HTTP headers, query parameters, or other protocol elements.

For example, this TLSN function redacts bytes from 2 to 4.

```js
redact(2, 4)
```

This low-level approach makes it possible to redact partial tokens or split meaningful fields across redaction boundaries. Let’s examine a specific case.


### Url Redaction

Consider the following redacted URL path:

```
/user?name=Jo*****rname=Smith
```

This redacted form could correspond to multiple original inputs, such as:

```
/user?name=John&surname=Smith
/user?name=JohnathansLongName
```

Without access to the hidden portion, it's impossible to determine which original URL the redacted version came from. This ambiguity arises because the redaction process operates on raw byte ranges of the same length, regardless of the underlying structure or semantics of the data.

### Enforcing URL Integrity

To guard against URL redaction issues, the Prover contract provides two verification modes. They limit the way url can be redacted.

#### 1. Full-URL verification

Use the `verify` function to check the integrity of the entire, unredacted URL. Example:

```solidity
function main(WebProof calldata webProof) {
    Web memory web = webProof.verify("example.com/user?name=John&surname=Smith")
    ...
}
```

This mode eliminates URL redaction issues by disallowing any redaction of the URL.

#### 2. URL prefix verification

Use the `verifyWithUrlPrefix` function to validate that the redacted URL starts with a known prefix. Example:

```solidity
function main(WebProof calldata webProof) {
    Web memory web = webProof.verifyWithUrlPrefix("example.com/user?name=")
    ...
}
```

* Assumes the prefix (“example.com/user?name=”) is correct.
* Treats everything after that prefix as untrusted.
* Ensures that sensitive suffix data (e.g. user IDs) remains hidden, while protecting contract logic from tampering.

Details on how to prepare a WebProof with redacted URL can be found [here](../javascript/web-proofs.md#url-redaction).

### Header Redaction

[PROBLEMATIC HEADER REDACTION EXAMPLE]

Both `verify` and `verifyWithUrlPrefix` functions handle header redactions in the same way. Details on how to prepare a WebProof with redacted headers can be found [here](../javascript/web-proofs.md#header-redaction).