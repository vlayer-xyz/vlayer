# JSON Web Tokens
Authentication with vlayer services is handled via **JSON Web Tokens (JWTs)**. Since JWTs are Base64-encoded by default, it can be difficult to visually inspect them when debugging failed authentication attempts.

For most use cases, generating a JWT through the [dashboard](https://dashboard.vlayer.xyz) is sufficient. However, for local development or debugging, the vlayer CLI includes a helpful utility for generating and verifying JWTs compatible with the vlayer network.

This utility is available under the `vlayer jwt` command.

## Generating/encoding new JWTs

Generating new JWTs can be done using `vlayer jwt encode` command.

Typical usage when targeting vlayer devnet would look as follows:

```bash
$ vlayer jwt encode -p docker/fixtures/jwt-authority.key --subject deadbeef --host "api.x.com" --port 443
2025-03-28T06:47:46.993637Z  INFO vlayer::commands::jwt: Claims {
    host: "api.x.com",
    port: 443,
    exp: 18446744073709551615,
    sub: "deadbeef",
}
2025-03-28T06:47:47.002804Z  INFO vlayer::commands::jwt: eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJob3N0IjoiYXBpLnguY29tIiwicG9ydCI6NDQzLCJleHAiOjE4NDQ2NzQ0MDczNzA5NTUxNjE1LCJzdWIiOiJkZWFkYmVlZiJ9.EPvz_8kHV1FW3SObjzmyN_WOCbmQvBNHHDsjMYd0M__lbfpeDUJinM8vLJ4KLYpo_nqEBipq4rl656pImXPNHKpmyrjWaEueG5bNY67Vyxa7A8B7jHkqnVpFe7L5kX-5M-kC-8JLxmlsNAMhm40vrmiC3uqFqnFAiXxefV-usnlGgnLMZSWfo5PwRFhayEsObHCJImsj5tKIUUS1d2dDzwRhBmrAIvOihbvAVnSQsrHTMxfs2-OsUQjRkDfEsBhz46Ei1fBRFoAj0-SQH04YBaWkQlNqStXOL2n_2eQyUnAJH_5sn7lSmXQLPhUlNHdh2Ly8DJ6qcZpGEoM1fKXL7nOIay5pEThGPqAZGiXL3yMt-E550EX_ccvTIYzSBqZ671Q4ziy1acNIWBsL5abm-Rui-crQHSXAH6q8ADQCEdMZT6jw6XeNxt-AQIh_GduVlIALoZBYrfsJi8MfcWsYwQ36TAzp67Wb7LmqXGWfVv0_XSNLjFMc-WGqTk195jfY-Sb8v11opF7BKEW1sH89ALCGX0dWMTablAzox8eKwawEZWmL0xEXQwqARqJD20EnWW6tZ3X8LV5JAVNhAfM6Yp9wJ_BWJmlN_5P7F6ODy_nTlr5tJ5yLpX2OLRQQrsIyRB-Y4VdH8K5riIIOPcjapXtIoqEaFNmkls5hX_3jzKk
```

Here we are generating a new JWT token with `subject` equal `deadbeef` and web-proof target host `api.x.com:443`.

We are also employing the default devnet private key to correctly sign the token stored at `docker/fixtures/jwt-authority.key`.

### Validating/decoding JWTs

Validating JWTs can be done using `vlayer jwt decode` command.

Decoding tokens that were signed using vlayer devnet's default private key would look as follows:

```bash
$ vlayer jwt decode -p docker/fixtures/jwt-authority.key.pub eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJob3N0IjoiYXBpLnguY29tIiwicG9ydCI6NDQzLCJleHAiOjE4NDQ2NzQ0MDczNzA5NTUxNjE1LCJzdWIiOiJkZWFkYmVlZiJ9.EPvz_8kHV1FW3SObjzmyN_WOCbmQvBNHHDsjMYd0M__lbfpeDUJinM8vLJ4KLYpo_nqEBipq4rl656pImXPNHKpmyrjWaEueG5bNY67Vyxa7A8B7jHkqnVpFe7L5kX-5M-kC-8JLxmlsNAMhm40vrmiC3uqFqnFAiXxefV-usnlGgnLMZSWfo5PwRFhayEsObHCJImsj5tKIUUS1d2dDzwRhBmrAIvOihbvAVnSQsrHTMxfs2-OsUQjRkDfEsBhz46Ei1fBRFoAj0-SQH04YBaWkQlNqStXOL2n_2eQyUnAJH_5sn7lSmXQLPhUlNHdh2Ly8DJ6qcZpGEoM1fKXL7nOIay5pEThGPqAZGiXL3yMt-E550EX_ccvTIYzSBqZ671Q4ziy1acNIWBsL5abm-Rui-crQHSXAH6q8ADQCEdMZT6jw6XeNxt-AQIh_GduVlIALoZBYrfsJi8MfcWsYwQ36TAzp67Wb7LmqXGWfVv0_XSNLjFMc-WGqTk195jfY-Sb8v11opF7BKEW1sH89ALCGX0dWMTablAzox8eKwawEZWmL0xEXQwqARqJD20EnWW6tZ3X8LV5JAVNhAfM6Yp9wJ_BWJmlN_5P7F6ODy_nTlr5tJ5yLpX2OLRQQrsIyRB-Y4VdH8K5riIIOPcjapXtIoqEaFNmkls5hX_3jzKk
2025-03-28T06:48:59.290970Z  INFO vlayer::commands::jwt: Header {
    typ: Some(
        "JWT",
    ),
    alg: RS256,
    cty: None,
    jku: None,
    jwk: None,
    kid: None,
    x5u: None,
    x5c: None,
    x5t: None,
    x5t_s256: None,
}
2025-03-28T06:48:59.292317Z  INFO vlayer::commands::jwt: Claims {
    host: "api.x.com",
    port: 443,
    exp: 18446744073709551615,
    sub: "deadbeef",
}
```