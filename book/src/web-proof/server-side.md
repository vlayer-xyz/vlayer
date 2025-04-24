# Server-side proving
vlayer supports generating Web Proofs on the server side. This method is especially useful for proving web data returned by public or token-authenticated APIs, without requiring a browser extension.

Typical use cases include verifying responses from centralized services such as Coinbase, Binance, Kraken, Stripe, Github, Meta, YouTube or similar platforms.

To notarize HTTP requests from the server, use the vlayer CLI:
```sh
vlayer web-proof-fetch [OPTIONS] --url <URL>
```

Available options: 
```sh
      --url <URL>            Full URL of the request to notarize
      --host <HOST>          Optional host address, if different from the domain provided in URL
      --notary <NOTARY_URL>  Notary URL [default: https://test-notary.vlayer.xyz/]
  -H, --headers <HEADER>     Additional headers (format: "Header-Name: Header-Value")
  -d, --data <DATA>          HTTP data to be sent with the request
  -h, --help                 Print help
  -V, --version              Print version
```

Example usage: 
```sh
vlayer web-proof-fetch 
  --notary "https://test-notary.vlayer.xyz" 
  --url "https://api.kraken.com/0/public/Ticker?pair=ETHUSD"
```

Such produced Web Proof (including url, headers and body) can be passed into vlayer prover and then verified on-chain. 

> 💡 **Try it Now**
>
> To run an example that proves data returned by the Kraken API, enter the following command in your terminal:
>
> ```bash
> vlayer init --template kraken-web-proof
> ```
>
> This will download all necessary artifacts to your project.  
> The next steps are detailed in [Running Examples](../getting-started/first-steps.md#running-examples-locally).

For proving HTTP requests that rely on httpOnly cookies, use the [client-side approach](/web-proof/quickstart-guide.html). 