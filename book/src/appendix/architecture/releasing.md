# Deployment and release

Updating `Verifier` infrastructure is a bit tricky:
- First, we need to deploy supporting contracts (i.e. Groth16 verifier)
- Than, ensure `GUEST_ID` in `Verifier` is updated. Note: this is done automatically by compilation process
- Then, modify address of main supporting contract variable in `Verifier`
- Release new version of vlayer solidity contracts library
- Release new version of vlayer node
