// import { createVlayerClient } from "@vlayer/sdk";

import { spawn } from "node:child_process";
import fs from "node:fs";
// Path to your Rust executable
const rustExecutable = "./tlsn-provider-rust/target/release/tlsn-provider"; // Adjust the path and name

async function runRustAndStreamOutput(url: string) {
  return new Promise<string>((resolve) => {
    const rustProcess = spawn(rustExecutable, [url]); // Pass the URL as an argument
    rustProcess.stdout.pipe(process.stdout);
    rustProcess.stderr.pipe(process.stderr);

    rustProcess.on("close", () => {
      const json = fs.readFileSync("./proof.json", "utf8");
      fs.unlinkSync("./proof.json");
      resolve(JSON.parse(json));
    });
  });
}

// Run the function
// runRustAndStreamOutput();
export const serverSideTlsnProofProvider = {
  getWebProof(url: string) {
    return runRustAndStreamOutput(url);
  },
};
