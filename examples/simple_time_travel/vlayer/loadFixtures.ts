import { $ } from "bun";
import { getConfig } from "@vlayer/sdk/config";

const config = getConfig();

await $`forge script --chain anvil scripts/LoadFixtures.s.sol:LoadFixtures --rpc-url ${config.jsonRpcUrl} --broadcast --private-key ${config.privateKey}`;
