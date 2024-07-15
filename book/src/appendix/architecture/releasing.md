# Deployment and Release

Updating the `Verifier` infrastructure can be a bit tricky:

1. First, deploy the supporting contracts (e.g., Groth16 verifier).
2. Then, ensure that the `GUEST_ID` in `Verifier` is updated. Note: this is done automatically by the compilation process.
3. Next, modify the address of the main supporting contract variable in `Verifier`.
4. After that, release a new version of the vlayer Solidity contracts library.
5. Finally, release a new version of the vlayer node.
