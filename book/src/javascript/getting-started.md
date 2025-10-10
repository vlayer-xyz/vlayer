## Getting Started

This documentation walks you through building a complete vlayer Web Proof application deployed to Optimism Sepolia testnet that:
1. Proves ownership of a Twitter/X account
2. Extracts the Twitter handle from the authenticated API response
3. Verifies the proof on-chain (Optimism Sepolia testnet)
4. Mints an NFT representing the verified Twitter account

Try out the fullstack demo [here](https://webproof-getting-started.vercel.app/).

## Environment Setup

### Step 1: Clone and Navigate

```bash
git clone https://github.com/vlayer-xyz/webproof-getting-started
cd webproof-getting-started
```

### Step 2: Install Contract Dependencies

```bash
cd contracts
forge soldeer install
forge build
```

This will:
- Install vlayer SDK (v1.5.1)
- Install OpenZeppelin contracts (v5.0.1)
- Install Forge Standard Library
- Compile all contracts

### Step 3: Install Deployment Dependencies

```bash
cd contracts/deploy
bun install
```

### Step 4: Configure Environment Variables

Create a `.env.testnet.local` file in `contracts/deploy/`:

```bash
cd contracts/deploy
touch .env.testnet.local
```

Add the following content:

```env
# Get this from https://dashboard.vlayer.xyz/
VLAYER_API_TOKEN="your_jwt_token_here"

# Your Ethereum wallet private key (must have Optimism Sepolia ETH)
EXAMPLES_TEST_PRIVATE_KEY="0xYourPrivateKeyHere"
```

**Getting Your vlayer API Token:**
1. Visit [vlayer Dashboard](https://dashboard.vlayer.xyz/)
2. Click "Create New Token" and check "Allow domain for webproofs"
3. Include the domain you want to prove for: `api.x.com`
4. Paste your `VLAYER_API_TOKEN` into `.env.testnet.local`

### Step 5: Deploy Contracts

Deploy to vlayer testnet (Optimism Sepolia):

```bash
cd contracts/deploy
bun run deploy:testnet
```

Expected output:
```
Deploying contracts...
Prover deployed at: 0x951a2e9612d6ace80ebd82f0f66a087c2932d31d
Verifier deployed at: 0x2462b1347212a4b956fd65a534b17b6a3a086418
Environment variables written to .env
```

### Step 6: Configure Frontend

Copy the generated `.env` file from `contracts/deploy/` to `react-frontend/`:

```bash
cp contracts/deploy/.env react-frontend/.env
```

Then edit `react-frontend/.env` to add your credentials:

```env
# Deployed contract addresses (auto-generated)
VITE_PROVER_ADDRESS=0x951a2e9612d6ace80ebd82f0f66a087c2932d31d
VITE_VERIFIER_ADDRESS=0x2462b1347212a4b956fd65a534b17b6a3a086418

# Network configuration (auto-generated)
VITE_CHAIN_NAME=optimismSepolia
VITE_PROVER_URL=https://stable-fake-prover.vlayer.xyz/1.5.1/
VITE_JSON_RPC_URL=https://sepolia.optimism.io
VITE_NOTARY_URL=https://test-notary.vlayer.xyz/v0.1.0-alpha.12
VITE_WS_PROXY_URL=wss://test-wsproxy.vlayer.xyz/jwt
VITE_GAS_LIMIT=5000

# Add these manually:
VITE_EXAMPLES_TEST_PRIVATE_KEY="0xYourPrivateKeyHere"
VITE_VLAYER_API_TOKEN="your_jwt_token_here"
```

> **⚠️ Warning:** Environment variables prefixed with `VITE_` are embedded into your frontend and publicly visible to anyone who accesses your application. Never use a wallet with real funds or your main private key. Create a dedicated test wallet with only testnet ETH for this tutorial.

### Step 7: Install and Run Frontend

```bash
cd react-frontend
bun install 
bun run dev 
```

Visit `http://localhost:5173` to see your application running.

## Smart Contract Development

### The Prover Contract

The prover contract (`WebProofProver.sol`) is responsible for verifying web proofs and extracting data.

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer-0.1.0/WebProof.sol";

contract WebProofProver is Prover {
    using WebProofLib for WebProof;
    using WebLib for Web;

    string public constant DATA_URL =
        "https://api.x.com/1.1/account/settings.json";

    function main(WebProof calldata webProof, address account)
        public
        view
        returns (Proof memory, string memory, address)
    {
        Web memory web = webProof.verifyWithUrlPrefix(DATA_URL);
        
        string memory screenName = web.jsonGetString("screen_name");
        
        return (proof(), screenName, account);
    }
}
```

#### Understanding the Prover

**Key Components:**

1. **Inheritance from `Prover`**
   - Provides access to `proof()` function
   - Enables zero-knowledge proof generation

2. **`DATA_URL` constant**
   - Specifies which API endpoint you're proving
   - Must match the URL in your frontend web proof request
   - Learn how to retrieve the API endpoint you want to prove [here](https://book.vlayer.xyz/web-proof/quickstart-guide.html#obtaining-web-proof)

3. **`main` function**
   - Entry point for proof generation
   - Takes a `WebProof` and user `address` as inputs
   - Returns a `Proof`, extracted data, and the address

4. **`verifyWithUrlPrefix()`**
   - Verifies the web proof signature
   - Checks that the notarized URL matches the expected prefix
   - Returns a `Web` object containing the verified data

5. **`jsonGetString()`**
   - Extracts a specific field from JSON response
   - In this case, gets the `screen_name` from Twitter's API

**Customization Points:**

To extract different data, you can use:

```solidity
// Extract a string
string memory value = web.jsonGetString("field_name");

// Extract a number
uint256 number = web.jsonGetUint("numeric_field");

// Extract from nested JSON
string memory nested = web.jsonGetString("user.profile.bio");
```

### The Verifier Contract

The verifier contract (`WebProofVerifier.sol`) validates proofs on-chain and executes business logic.

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {WebProofProver} from "./WebProofProver.sol";
import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Verifier} from "vlayer-0.1.0/Verifier.sol";
import {ERC721} from "@openzeppelin-contracts-5.0.1/token/ERC721/ERC721.sol";

contract WebProofVerifier is Verifier, ERC721 {
    address public prover;
    
    mapping(uint256 => string) public tokenIdToMetadataUri;
    
    constructor(address _prover) ERC721("TwitterNFT", "TNFT") {
        prover = _prover;
    }
    
    function verify(Proof calldata, string memory username, address account)
        public
        onlyVerified(prover, WebProofProver.main.selector)
    {
        uint256 tokenId = uint256(keccak256(abi.encodePacked(username)));
        require(_ownerOf(tokenId) == address(0), "User has already minted a TwitterNFT");
        
        _safeMint(account, tokenId);
        tokenIdToMetadataUri[tokenId] = string.concat(
            "https://faucet.vlayer.xyz/api/xBadgeMeta?handle=", 
            username
        );
    }
    
    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        return tokenIdToMetadataUri[tokenId];
    }
}
```

#### Understanding the Verifier

**Key Components:**

1. **Multiple Inheritance**
   - `Verifier`: Provides proof verification functionality
   - `ERC721`: Standard NFT implementation

2. **`onlyVerified` modifier**
   - Automatically verifies the zero-knowledge proof
   - Ensures the proof was generated by the correct prover contract
   - Validates the proof corresponds to the correct function (`main.selector`)

3. **Proof parameters**
   - First parameter is always `Proof calldata`
   - Subsequent parameters must match prover's return values (excluding the Proof)
   - In this case: `string memory username, address account`

4. **Business Logic**
   - Creates deterministic token ID from username hash
   - Prevents duplicate minting (one NFT per Twitter handle)
   - Mints NFT to the proven account owner
   - Sets metadata URI dynamically

## Frontend Integration

### Setting Up the vlayer Client

The [frontend integration](https://github.com/vlayer-xyz/webproof-getting-started/blob/main/react-frontend/src/App.jsx) involves three main steps:
1. Creating a Web Proof Provider
2. Initializing the vlayer Client
3. Creating and executing Web Proof Requests

#### Step 1: Web Proof Provider

```javascript
const webProofProvider = createExtensionWebProofProvider();
```

This creates a provider that interfaces with the vlayer browser extension. The extension will:
- Open when a proof is requested
- Guide the user through the proof generation process
- Capture and notarize web traffic

#### Step 2: vlayer Client

```javascript
const vlayer = createVlayerClient({
  url: import.meta.env.VITE_PROVER_URL,
  token: import.meta.env.VITE_VLAYER_API_TOKEN,
  webProofProvider: webProofProvider,
});
```

The client connects to vlayer's proving infrastructure and authenticates with your API token.

#### Step 3: Web Proof Request

```javascript
const webProofRequest = createWebProofRequest({
  steps: [
    startPage('https://x.com/home', 'Go to x.com login page'),
    expectUrl('https://x.com/home', 'Log in'),
    notarize('https://api.x.com/1.1/account/settings.json', 'GET', 'Generate Proof'),
  ],
});
```

**Step Types:**

- `startPage(url, instruction)`: Opens a URL and displays an instruction
- `expectUrl(url, instruction)`: Waits for user to navigate to a specific URL
- `notarize(url, method, instruction)`: Captures and notarizes a specific API call

**Best Practices:**
- Keep instructions clear and user-friendly
- Match the notarize URL to your prover contract's `DATA_URL`

#### Step 4: Generate Proof

```javascript
const hash = await vlayer.proveWeb({
  address: import.meta.env.VITE_PROVER_ADDRESS,
  proverAbi: WebProofProver.abi,
  functionName: 'main',
  args: [webProofRequest, targetAddress],
  chainId: optimismSepolia.id,
});
```

This initiates the proving process:
1. Opens the browser extension
2. User follows the guided steps
3. Web traffic is captured and notarized
4. Returns a hash to track the proving job

#### Step 5: Wait for Result

```javascript
const result = await vlayer.waitForProvingResult({ hash });
```

This polls the proving service until the zero-knowledge proof is generated. The result contains all the arguments needed to call your verifier contract.

#### Steps 6-8: On-Chain Verification

These steps use standard Viem patterns to:
1. Create wallet clients
2. Simulate the transaction (optional but recommended)
3. Submit the verification transaction

## Conclusion

Congrats! You've built a complete Web Proof application that proves Twitter/X account ownership, extracts authenticated data, and mints NFTs on Optimism Sepolia. You now understand how to create prover contracts that verify web data, verifier contracts that execute on-chain logic, and frontend integrations that guide users through the proof generation process. From here, you can adapt this pattern to prove data from any web API, whether it's verifying GitHub contributions, proving DeFi positions, or authenticating any other web-based credential.