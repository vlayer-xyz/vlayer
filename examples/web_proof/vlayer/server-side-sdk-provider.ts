// import { createVlayerClient } from "@vlayer/sdk";

import { spawn } from "node:child_process";
import fs from "node:fs";
// Path to your Rust executable
const rustExecutable = "./tlsn-provider-rust/target/release/tlsn-provider"; // Adjust the path and name

async function runRustAndStreamOutput(url: string) {
  return new Promise<string>((resolve) => {
    const rustProcess = spawn(rustExecutable, [url]); // Pass the URL as an argument

    rustProcess.on("close", (code) => {
      console.log(`Rust process exited with code ${code}`);
      const json = fs.readFileSync("./tlsn-provider-rust/proof.json", "utf8");
      console.log("json", json);
      resolve(json);
    });
  });
}

// Run the function
// runRustAndStreamOutput();
const serverSideTlsnProofProvider = {
  getWebProof(url: string) {
    return runRustAndStreamOutput(url);
  },
};

serverSideTlsnProofProvider.getWebProof(
  " https://www.accountable.capital:10443/binance",
);
