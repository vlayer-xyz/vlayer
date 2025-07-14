# simple-web-proof test data

This directory contains test fixtures for web proofs.

These files are used to mock parts of the proof flow, allowing us to demonstrate and test the process in Node.js - without needing a full browser environment or extension.

## Fixtures

### `web_proof_development_signature.json`

Contains a web proof of X account ownership, signed by a development notary key that is publicly known.

Web proofs signed by this key are accepted by Verifier contracts running on development chains (such as Anvil) and on testnet chains.

### `web_proof_vlayer_signature.json`

This is a web proof of X account ownership generated using vlayer's notary deployment.

Mainnet Verifier contracts will only accept web proofs signed with this key.

### `web_proof_invalid_signature.json`

This is a valid web proof, but generated using a different, unsupported notary key.

It will be rejected by all Verifier contracts.
