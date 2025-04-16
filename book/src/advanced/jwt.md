# JSON Web Tokens

Authentication into vlayer services is done using JSON Web Tokens (JWTs). JWTs are base64
encoded by default which may be difficult to simply eye-ball when searching for the reason
that your request failed to authenticate.

This is particulary important when developing with [vlayer's devnet](/getting-started/dev-and-production.html)
and you would like to generate/validate different JWT combinations.

For that reason, we have provided a tool that's integrated with the vlayer CLI which allows
generating and verifying JWT tokens that are compatible with the vlayer network.

The tool can be accessed under `vlayer jwt` command.

## Generating/encoding new JWTs

Generating new JWTs can be done using `vlayer jwt encode` command.

Typical usage when targeting vlayer devnet would look as follows:

```bash
$ vlayer jwt encode -p docker/fixtures/jwt-authority.key --subject deadbeef --host "api.x.com" --post 443
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
