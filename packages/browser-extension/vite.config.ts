/// <reference types="vitest/config" />
import { defineConfig } from "vite";
import webExtension, { readJsonFile } from "vite-plugin-web-extension";
import { viteStaticCopy } from "vite-plugin-static-copy";
import tsconfigPaths from "vite-tsconfig-paths";
import { nodePolyfills } from "vite-plugin-node-polyfills";

function generateManifest() {
  const manifest = readJsonFile("src/manifest.json") as object;
  const pkg = readJsonFile("package.json") as {
    name: string;
    version: string;
    description: string;
  };
  return {
    name: pkg.name,
    description: pkg.description,
    version: pkg.version.split("-")[0], // The version in manifest.json can only have numbers.
    version_name: pkg.version,
    ...manifest,
  };
}

export default defineConfig({
  test: {
    environment: "jsdom",
    include: ["src/**/*.test.ts", "src/**/*.test.tsx"],
    setupFiles: [
      "./vitest/setup.ts",
      "./vitest/custom.matchers.ts",
      "@vitest/web-worker",
    ],
  },
  build: {
    minify: false,
    terserOptions: { compress: false, mangle: false },
  },
  plugins: [
    tsconfigPaths(),
    webExtension({
      manifest: generateManifest,
      watchFilePaths: ["package.json", "manifest.json"],
      webExtConfig: {
        startUrl: "http://localhost:5174",
        target: "chromium",
      },
    }),
    // copying is needed due to tlsn-js path resolution.
    viteStaticCopy({
      targets: [
        {
          src: `${__dirname}/../../node_modules/tlsn-js/build/**`,
          dest: "./",
        },
        {
          src: `${__dirname}/../../node_modules/tlsn-js/build/**`,
          dest: "assets/",
        },
        {
          src: `${__dirname}/../../node_modules/tlsn-js/build/**`,
          dest: "src/hooks/tlsnProve/",
        },
      ],
    }),
    nodePolyfills({
      // buffer is required by tlsn-js internals
      include: ["buffer"],
    }),
  ],
});
