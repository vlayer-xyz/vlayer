# Redaction

The TLSN protocol allows for redacting (hiding) parts of the HTTPS transcript from the `Prover`—i.e., excluding certain sensitive parts (e.g., cookies, authorization headers, API tokens) from the generated Web Proof—while still cryptographically proving that the rest of the transcript (the revealed parts) is valid.

Different redaction modes have important security implications you should understand. Learn more in the [Security model](#security-model) section.

To learn how to enable and configure redaction using the vlayer SDK, see the [Redaction](../../javascript/web-proofs.md#redaction) section in our JavaScript documentation.

---

## Partial redaction

Each value must be either fully redacted or not redacted at all. The Solidity methods `webProof.verify(Url)` and `webProof.verifyWithUrlPrefix(UrlPrefix)` validate that these conditions are met. This ensures that the structure of the transcript cannot be altered by a malicious client.

After redacting a JSON string value for a given `"key"`, `web.jsonGetString("key")` returns a string in which each byte is replaced by the `*` character.

---

## Security model

A limitation of the current redaction process is that it does not incorporate HTTP semantics. In TLSN, redaction operates on raw byte ranges rather than structured protocol elements.

For example:

```js
redact(2, 4)
```

This low-level approach makes it possible to redact partial tokens or split meaningful fields across redaction boundaries.

Consider the following path:

```
/user?name=John&surname=Smith
```

could be redacted with `js` as:

```
/user?name=Jo*****rname=Smith
```

If a redaction is applied without awareness of parameter structure, it may inadvertently redact only part of a value or key, breaking the semantics of the query string. Although such partial redactions are rejected by vlayer, there are edge cases we can't detect because the original values are hidden, and the redacted version appears valid. For instance:

```
/user?name=******************
```

is indistinguishable from a valid redaction.

### Implications

This means a malicious actor can:

- Remove query parameters
- Remove JSON fields
- Change JSON structure so that a field value appears under a different key
  - Fields can be moved both up and down the tree

---

## How to mitigate risks

### Request headers, body, and response headers

We do **not** expose these on the Web object, so even if they are parsed incorrectly, the Prover cannot access them.

---

### Request URL

- **If you can avoid redacting the URL**:  
  Set `UrlTestMode` to `Full` and use:

  ```js
  verify(Url)
  ```

- **If you must redact the URL**:  
  Set `UrlTestMode` to `Prefix` and use:

  ```js
  verifyWithUrlPrefix(UrlPrefix)
  ```

  You must treat all characters after the first redaction character as untrusted, since they can be manipulated.

---

### Response JSON body

- **If you can avoid redaction**:  
  Set:

  ```js
  BodyRedactionMode = "Disabled"
  ```

  This allows TLSN to verify that the body is valid, well-formed JSON.

- **If you must redact JSON body**:  
  Set:

  ```js
  BodyRedactionMode = "Enabled_UNSAFE"
  ```

  “UNSAFE” has a Rust-like meaning: **you** are responsible for ensuring safety. You **must** check:

  - All fields in your JSON schema are required (no optional fields)
  - Arrays have fixed, known-in-advance sizes (to avoid `[1, 2, 3]` ⇒ `[1, ****]`)
  - All required fields are present

These rules help prevent structural manipulation attacks, such as injecting or hiding malicious fields.

Use this mode **only as a last resort**, as it’s hard to apply safely in real-world production data.
