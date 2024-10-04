import { defineConfig } from "vite";
import webExtension, { readJsonFile } from "vite-plugin-web-extension";
import { viteStaticCopy } from "vite-plugin-static-copy";
import tsconfigPaths from "vite-tsconfig-paths";
import path from "node:path";

function generateManifest() {
  const manifest = readJsonFile("src/manifest.json");
  const pkg = readJsonFile("package.json");
  return {
    name: pkg.name,
    description: pkg.description,
    version: pkg.version.split("-")[0], // The version in manifest.json can only have numbers.
    version_name: pkg.version,
    ...manifest,
  };
}

export default defineConfig({
  plugins: [
    tsconfigPaths(),
    webExtension({
      manifest: generateManifest,
      watchFilePaths: ["package.json", "manifest.json"],
      webExtConfig: {
        startUrl: "http://localhost:5174",
      },
    }),
    viteStaticCopy({
      targets: [
        {
          src: `${__dirname}/../node_modules/tlsn-js/build/284ddec2a9dac2774b1d.wasm`,
          dest: "src/templates/sidepanel",
        },
        {
          src: `${__dirname}/../node_modules/tlsn-js/build/760.js`,
          dest: "src/templates/sidepanel",
        },
      ],
    }),
  ],
});
