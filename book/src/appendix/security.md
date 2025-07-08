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

The Smart Contracts provided by vlayer provide base classes from which the Developer derives their own Smart Contracts.

Only testnet contracts allow fake proofs - on mainnet, only real Groth16 proofs are valid.

### Mainnet Addresses

| Contract               | Address                                      |
|------------------------|----------------------------------------------|
| Repository             | `0x42fc5CdBfA5E4699C0e1e0adD0c4BC421d80482F` |
| Groth16ProofVerifier   | `0xb8Be5BdCD6387332448f551cFe7684e50d9E108C` |

### Testnet addresses

| Contract               | Address                                      |
|------------------------|----------------------------------------------|
| Repository             | `0xAD04462241343F0775315B2873E6fe6DffCce831` |
| Groth16ProofVerifier   | `0x074Fc67dA733FFA5B288a9d5755552e61a1A0a06` |
| FakeProofVerifier      | `0xeF2f0Cbb90821E1C5C46CE5283c82F802F65a3f3` |
| ProofVerifierRouter    | `0x7d441696a6F095B3Cd5e144ccBCDB507e0ce124e` |
