/// <reference types="vitest/config" />

import { defineConfig } from "vite";
import tsconfigPaths from "vite-tsconfig-paths";

export default defineConfig({
  plugins: [tsconfigPaths()],
  build: {
    target: "esnext",
  },
  // This is added as a temporary fix for the `process is not defined` issue
  // (https://github.com/reown-com/appkit/issues/3926)
  // appearing in our dependency: reown/appkit
  define: {
    "process.env": {},
  },
  test: {
    environment: "jsdom",
    include: ["./tests/**/*.unit.ts", "./tests/**/*.unit.tsx"],
    setupFiles: ["./tests/vitest.setup.ts"],
  },
});
