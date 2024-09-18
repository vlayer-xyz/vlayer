import { defineConfig } from "vite";
import webExtension, { readJsonFile } from "vite-plugin-web-extension";
import { viteStaticCopy } from "vite-plugin-static-copy";

function generateManifest() {
  const manifest = readJsonFile("src/manifest.json");
  const pkg = readJsonFile("package.json");
  return {
    name: pkg.name,
    description: pkg.description,
    version: pkg.version,
    ...manifest,
  };
}

export default defineConfig({
  plugins: [
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
