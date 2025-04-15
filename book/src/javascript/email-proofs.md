# Email proofs from SDK
## Email Proofs

In order to prove the content of an email, we firstly need to prepare it to be passed into the smart contract. 
We provide a handy function for it in the SDK, `preverifyEmail`. 

```ts
import fs from "fs";
import { preverifyEmail } from "@vlayer/sdk";
// .. Import prover contract ABI

// Read the email MIME-encoded file content
const email = fs.readFileSync("email.eml").toString();

// Prepare the email for verification
const unverifiedEmail = await preverifyEmail(email);

// Create vlayer server client
const vlayer = createVlayerClient();

const hash = await vlayer.prove({
  address: prover,
  proverAbi: emailProofProver.abi,
  functionName: "main",
  args: [unverifiedEmail],
  chainId: foundry,
});
const result = await vlayer.waitForProvingResult({ hash });
```

The `email.eml` file should be a valid email. Usually it can be exported from your email client.

<div class="warning">
The email cannot be modified in any way (including whitespaces and line breaks), 
because it will make the signature invalid.
</div>

