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
| Repository             | `0x9DFcaBBDbe296E5C4bf36E4431C46daB43702c84` |
| Groth16ProofVerifier   | `0xc4B7dEd1C30ec34802c85B8345eaC15c02d646A0` |

### Testnet addresses

| Contract               | Address                                      |
|------------------------|----------------------------------------------|
| Repository             | `0xE44007361170f82a5DAB427905Dd355E2CbeE7dB` |
| Groth16ProofVerifier   | `0xBabf9630bA994902E57f71Db032771f53B16C35b` |
| FakeProofVerifier      | `0x3bE01bee0cA51f5a84Ba10e675dD7576E20429CA` |
| ProofVerifierRouter    | `0xaB0B39778577A2536f12E53db7932859e1743605` |
