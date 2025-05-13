<img src="images/cover.jpg" style="border-radius: 20px" alt="Trustless verifiable data infrastructure powered by zero-knowledge proofs">

# Introduction

vlayer enables developers to extract, verify and integrate real-world data into Ethereum smart contracts. Our technology is powered by Zero Knowledge Proofs (ZKP) and Multi-Party Computation (MPC), allowing you to securely verify private data without exposing sensitive information. 

Our four core features include: 
- [Web Proofs](/features/web.html): Access verified web data, including APIs and websites, in your smart contracts 
- [Email Proofs](/features/email.html): Tap into email content from your smart contracts and use it on-chain
- [Time Travel](/features/time-travel.html): Leverage historical on-chain data in your smart contracts
- [Teleport](/features/teleport.html): Execute a smart contract across different EVM-comptable blockchain networks

vlayer allows smart contracts to be executed [off-chain](/advanced/prover.html). The result of the execution can then be used by [on-chain contracts](/advanced/verifier.html).

## What are some real-world applications of vlayer?
vlayer offers diverse applications across industries that demand secure and privacy-preserving data verification. By enabling users to prove specific facts without revealing underlying personal information, vlayer empowers businesses to build trust without the need to access or store sensitive data like names, birthdates, government IDs, or financial information. 

A few real-world applications of vlayer include: 
- Use **Web Proofs** to verify social media engagement for brand partnerships or to generate verifiable proof of asset custody in institutional holdings 
- Use **Email Proofs** to reset account abstraction wallets 
- Use **Time Travel** to verify historical ETH, ERC-721, or ERC-20 balances for airdrops, voting rights, or other on-chain entitlements
- Use proof of holdings on another blockchain as collateral for loans via **Teleport** 

Additional use cases can be found [here](https://vlayer.notion.site/hacker-house-ideas).

### Sections
**Getting Started**

To get started with vlayer, [install vlayer](/getting-started/installation.html), set up your [first project](/getting-started/first-steps.html) and check out the explainer section to learn [how vlayer works](/getting-started/how-it-works.html). Finally take a look into [devnet, testnet & mainnet](/getting-started/dev-and-production.html) to learn about vlayer environments. 

**Features**

See how to [time travel](/features/time-travel.html) across block numbers, [teleport](/features/teleport.html) from one chain to another, prove data coming from [email](/features/email.html) or [web](/features/web.html) and use helpers for [JSON and Regex](/features/json-and-regex.html).

**From JavaScript**

Learn how to [interact with vlayer](/javascript/javascript.html) from your JS code and how to [generate web proofs](/javascript/web-proofs.html) and [email proofs](/javascript/email-proofs.html) using our SDK.

**Advanced**

Learn in-depth how:
 * [Prover](/advanced/prover.html) and [Verifier](/advanced/verifier.html) contracts are working.
 * [Global Variables](./advanced/prover-global-variables.md) are set.
 * [Tests](/advanced/tests.html) are run.
