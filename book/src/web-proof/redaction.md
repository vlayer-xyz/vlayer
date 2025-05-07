# Redaction

The core limitation of redaction in TLSN is that it's not semantics-aware.

Redaction in TLSN works on the level of byte ranges.
E.g. `redact(2, 4)`.

In a perfect world - redaction would be aware of the structure of HTTP request/response and be available through a semantics API like: `redactRequestHeader("User-Agent)` or `redactResponseJSONBodyField("sender.uuid")`.

Therefore - user can spill redaction over the lexer tokens borders. E.g. `/user?name=John&surname=Smith` can be redacted as: `/user?name=Jo*****rname=Smith`. And while it only makes sense to redact full values and the example above is rejected by us as invalid because of partial redaction - there are examples that we can't reject as we don't see the values before redaction and redacted value looks legit. 

`/user?name=******************` is indistinguishable from a valid redaction.

So now we see that you can make it seem like query parameter is not there. In fact - you can do much more. Some notable examples are:
* Remove query parameters
* Remove JSON fields
* Change JSON structure so that a field value appears to be under a different key
    * You can move fields both up and down the tree

Example for JSON body:
```json
{
    outer: "",
    nested: {
        first: "malicious_value"
    },
    first: "real_value",
    second: ""
}
```

And redacted
```json
{
    outer: "**
*************
********first: "malicious_value"
******
************************
    second: ""
}
```

If the Prover later queries `.first` it will get `malicious_value`. The example above might not be pixel-perfect but you get the point.

There are a couple of ways to avoid that depending on the type of data:
* For request headers, request body, response headers - we don't expose it on the Web object - so the Prover can't access them even if they are parsed wrongly
* For request URL - every character of the request after the first redaction character is untrusted (see query params example above). Therefore:
    * If you can live without redaction in URL - disable it by setting `UrlTestMode` to `Full` and use normal `verify(Url)` function
    * if you need it (e.g. have private data in url params or path) - enable it by setting `UrlTestMode` to `Prefix`. Instead of `verify(Url)` you'll need to use `verifyWithUrlPrefix(UrlPrefix)`. You should not rely on parts of URL after redaction as they can be manipulated.
* For response JSON body:
    * Again - if you can live without redaction in response body - set `BodyRedactionMode` to `Disabled`. That way we can check that body is a correct well-formed JSON with no redaction.
    * if you need body redaction - you can enable it by setting `BodyRedactionMode` to `Enabled_UNSAFE`. `UNSAFE` here has a Rust-like meaning as in - you are responsible for making it safe. In order for it to be safe - you'll need to check a couple of invariants:
        * All values in the JSON-schema of your response must be required (no optional fields)
        * This implies that all arrays must have constant known in advance size. Otherwise, you can do this: `[1, 2, 3]` -> `[1, ****]` which will make it possible to manipulate array length and by index access.
        * You must check that all fields are in fact present. This will prevent attacks like the one listed above on JSON as `nested.first` will not be present and the Prover will fail.
    * As you can see - second option contain rigorous requirements that are hard to satisfy on real production data, so it must be used as the last resort.

## Partial redaction

Each value must be redacted fully or not at all. The Solidity method `webProof.verify(Url)` or `webProof.verifyWithUrlPrefix(UrlPrefix)` validates that these conditions are met. This way we ensure that the structure of the transcript cannot be altered by a malicious client. After redacting JSON string value for a given `"key"`, `web.jsonGetString("key")` returns a string with each byte replaced by `*` character.
