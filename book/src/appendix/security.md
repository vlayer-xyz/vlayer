# Security

vlayer takes security seriously and undergoes regular audits by reputable third-party firms.

## Audits

- [v1.0 audit by Veridise - Q2 2025](/static/audits/audit-2025-q2-veridise.pdf)

## Multisigs

The on-chain Smart Contracts are secured by the industry standard [Safe](https://safe.global/wallet) multisigs.

Addresses:

| Network   | Address                                    | Explorer Link                                                                                              |
|-----------|--------------------------------------------|------------------------------------------------------------------------------------------------------------|
| Ethereum  | 0xb06DB7A76f861874a2634200029BbE63Af5Be7CC | [Etherscan](https://etherscan.io/address/0xb06DB7A76f861874a2634200029BbE63Af5Be7CC)                       |
| Optimism  | 0x6924C300F8b7751f8c1Fa5A14e674A5B519645E1 | [Optimistic Etherscan](https://optimistic.etherscan.io/address/0x6924C300F8b7751f8c1Fa5A14e674A5B519645E1) |
| Base      | 0xBa8aA379D4Bf594b70Da1C414c5765ebC8223174 | [Basescan](https://basescan.org/address/0xBa8aA379D4Bf594b70Da1C414c5765ebC8223174)                        |

## Verifier Smart Contracts

vlayer provides base classes from which the developer derives their own smart contracts.

> Note: On testnet, contracts support [fake proofs](../getting-started/dev-and-production.md#fake-mode) for faster development and testing.
>
> On mainnet, only [real Groth16 proofs](../getting-started/dev-and-production.md#groth16-mode) are valid.

### Mainnet Addresses

| Contract               | Address                                      |
|------------------------|----------------------------------------------|
| Repository             | `0xbDf27a6f3CF309F9127d8173d0D28bF9ab35ed2b` |
| Groth16ProofVerifier   | `0x1EE8a3B907EbcdFc33f76e3C7aAe6FFD2eFA5b73` |

### Testnet addresses

| Contract               | Address                                      |
|------------------------|----------------------------------------------|
| Repository             | `0x0cFfdB4e737F00Ef57b4c61dBfBb334B3a416519` |
| Groth16ProofVerifier   | `0x9AdE0B5F34402AeFdcBE1a8733d5995Ff827f586` |
| FakeProofVerifier      | `0x711B293738290768f3eD1DBf2D00e0f9eEc19E6B` |
| ProofVerifierRouter    | `0x7925a78734Fc7f2cb69d7E03d81467BB851f9Eb8` |
