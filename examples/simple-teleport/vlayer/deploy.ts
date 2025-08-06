import proverSpec from "../out/SimpleTeleportProver.sol/SimpleTeleportProver.json";
import verifierSpec from "../out/SimpleTeleportVerifier.sol/SimpleTeleportVerifier.json";
import whaleBadgeNFTSpec from "../out/WhaleBadgeNFT.sol/WhaleBadgeNFT.json";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  waitForContractDeploy,
  writeEnvVariables,
} from "@vlayer/sdk/config";
import { loadFixtures } from "./loadFixtures";
import { getTeleportConfig } from "./constants";
import { type Address } from "viem";
import fs from "fs";
import path from "path";

const config = getConfig();
const teleportConfig = getTeleportConfig(config.chainName);

if (config.chainName === "anvil") {
  await loadFixtures();
}

const { ethClient, account } = createContext(config);

if (!account) {
  throw new Error(
    "No account found make sure EXAMPLES_TEST_PRIVATE_KEY is set in your environment variables",
  );
}

console.log("⏳ Deploying helper contracts...");
console.log(`   Chain: ${config.chainName}`);
console.log(`   RPC URL: ${config.jsonRpcUrl}`);
console.log(`   Account: ${account.address}`);
console.log(`   Gas Limit: ${config.gasLimit}`);

console.log("   📦 Deploying WhaleBadgeNFT contract...");
const deployWhaleBadgeHash = await ethClient.deployContract({
  abi: whaleBadgeNFTSpec.abi,
  bytecode: whaleBadgeNFTSpec.bytecode.object,
  account,
});

console.log(`   ✅ WhaleBadgeNFT deployment hash: ${deployWhaleBadgeHash}`);
console.log("   ⏳ Waiting for WhaleBadgeNFT deployment confirmation...");

const whaleBadgeNFTAddress = await waitForContractDeploy({
  client: ethClient,
  hash: deployWhaleBadgeHash,
});

console.log(`   ✅ WhaleBadgeNFT deployed at: ${whaleBadgeNFTAddress}`);

console.log("   📦 Deploying Vlayer contracts (Prover & Verifier)...");
const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [],
  verifierArgs: [whaleBadgeNFTAddress],
});

console.log(`   ✅ Prover deployed at: ${prover}`);
console.log(`   ✅ Verifier deployed at: ${verifier}`);

console.log("   📝 Preparing tokens configuration...");
const tokensToCheck: {
  addr: Address;
  chainId: bigint;
  blockNumber: bigint;
  balance: bigint;
}[] = (teleportConfig.prover.erc20Addresses.split(",") || []).map(
  (addr, i) => ({
    addr: addr as `0x${string}`,
    chainId: BigInt(teleportConfig.prover.erc20ChainIds.split(",")[i]),
    blockNumber: BigInt(teleportConfig.prover.erc20BlockNumbers.split(",")[i]),
    balance: BigInt(0),
  }),
);

console.log(`   ✅ Configured ${tokensToCheck.length} tokens to check`);
tokensToCheck.forEach((token, i) => {
  console.log(`      Token ${i + 1}: ${token.addr} (Chain: ${token.chainId}, Block: ${token.blockNumber})`);
});

console.log("   📝 Writing environment variables...");
await writeEnvVariables(".env", {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
  VITE_CHAIN_NAME: config.chainName,
  VITE_PROVER_URL: config.proverUrl,
  VITE_VLAYER_API_TOKEN: config.token,
  VITE_TOKENS_TO_CHECK: `"${JSON.stringify(tokensToCheck, (key, value) => 
    typeof value === 'bigint' ? value.toString() : value
  )}"`,
  VITE_DEFAULT_TOKEN_HOLDER: teleportConfig.tokenHolder,
  VITE_GAS_LIMIT: config.gasLimit,
});

console.log("   ✅ Environment variables written to .env");

// Update constants.ts with deployed addresses
console.log("   📝 Updating constants.ts with deployed addresses...");
await updateConstantsFile(config.chainName, {
  prover,
  verifier,
  whaleBadgeNFT: whaleBadgeNFTAddress,
  tokenHolder: teleportConfig.tokenHolder,
  tokensToCheck,
});

console.log("✅ Deployment complete! Contracts deployed and constants updated.");
console.log("   🎯 Next steps:");
console.log("      - Run 'bun run prove.ts' to execute the proving process");
console.log("      - Or run 'bun run web:testnet' to start the web interface");

// Function to update constants.ts file
async function updateConstantsFile(
  chainName: string,
  deployedAddresses: {
    prover: string;
    verifier: string;
    whaleBadgeNFT: string;
    tokenHolder: string;
    tokensToCheck: Array<{
      addr: Address;
      chainId: bigint;
      blockNumber: bigint;
      balance: bigint;
    }>;
  }
) {
  console.log(`      📂 Reading constants.ts file...`);
  const constantsPath = path.join(process.cwd(), "constants.ts");
  let constantsContent = fs.readFileSync(constantsPath, "utf8");

  console.log(`      📝 Creating new configuration for ${chainName}...`);
  // Create the new config object
  const newConfig = {
    tokenHolder: deployedAddresses.tokenHolder,
    prover: {
      erc20Addresses: deployedAddresses.tokensToCheck.map(t => t.addr).join(","),
      erc20ChainIds: deployedAddresses.tokensToCheck.map(t => t.chainId.toString()).join(","),
      erc20BlockNumbers: deployedAddresses.tokensToCheck.map(t => t.blockNumber.toString()).join(","),
    },
  };

  // Convert the config to a formatted string
  const configString = JSON.stringify(newConfig, null, 2)
    .replace(/"([^"]+)":/g, "$1:") // Remove quotes from keys
    .replace(/"/g, '"') // Keep quotes around string values
    .replace(/"([^"]+)"/g, '"$1"'); // Ensure proper string formatting

  console.log(`      🔍 Looking for existing ${chainName} configuration...`);
  // Find the existing chain configuration and replace it
  const chainConfigRegex = new RegExp(
    `(${chainName}:\\s*{[^}]+})`,
    "s"
  );

  if (chainConfigRegex.test(constantsContent)) {
    console.log(`      🔄 Replacing existing ${chainName} configuration...`);
    // Replace existing configuration
    constantsContent = constantsContent.replace(
      chainConfigRegex,
      `${chainName}: ${configString}`
    );
  } else {
    console.log(`      ➕ Adding new ${chainName} configuration...`);
    // Add new configuration before the closing brace of chainToTeleportConfig
    const insertIndex = constantsContent.lastIndexOf("};");
    if (insertIndex !== -1) {
      constantsContent = constantsContent.slice(0, insertIndex) +
        `,\n  ${chainName}: ${configString}\n` +
        constantsContent.slice(insertIndex);
    }
  }

  console.log(`      💾 Writing updated constants.ts file...`);
  // Write the updated content back to the file
  fs.writeFileSync(constantsPath, constantsContent, "utf8");
  
  console.log(`   ✅ Updated constants.ts for chain: ${chainName}`);
  console.log(`      Prover: ${deployedAddresses.prover}`);
  console.log(`      Verifier: ${deployedAddresses.verifier}`);
  console.log(`      WhaleBadgeNFT: ${deployedAddresses.whaleBadgeNFT}`);
  console.log(`      Token Holder: ${deployedAddresses.tokenHolder}`);
}
