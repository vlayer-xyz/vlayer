# Email proofs from SDK
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

const {
  proof,
  returnValue: [result],
} = await prove(prover, emailProofProver.abi, "main", [unverifiedEmail]);
```

The `email.eml` file should be a valid email, probably exported from your email client.

<div class="warning">
The email cannot be modified in any way (including whitespaces and line breaks), 
because it will make the signature invalid.
</div>

