# Email Proofs Architecture

## Email proof sequence flow

![Architecture diagram](../../images/architecture/email-sequence.png)

Generating and ZK-proving an Email Proof consists of the following steps:
1. Received email MIME file is extracted from the email client.
2. The preverifyEmail in SDK prepares the UnverifiedEmail struct that is ready to be sent to the Prover Contract.
    - performs basic preverification - checks if DKIM-Signature header is present
    - calls the `DNS Notary` to get the verification data of the sender's domain
    - Note that all these steps can be performed without the vlayerSDK.
3. Do a `v_call` to the vlayer prover server with the UnverifiedEmail struct as calldata. Prover contract address and chain ID
4. Prover contract must use the EmailProofLib, where the `DNS Notary` public key is verified via the `Repository` contract, preverification ttl is verified against block number and email verification precompiled is triggered
5. The EmailProofLib contract calls the `email_proof.verify()` custom precompile (see [Precompiles](./prover.md#precompiles)), which validates the Email Proof, parses the email MIME file and returns `VerifiedEmail`.
6. If the verification is successful, the EmailProofLib contract returns the ZK Proof and the public returned values.