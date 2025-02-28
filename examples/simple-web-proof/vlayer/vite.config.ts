import { defineConfig } from "vite";
import tsconfigPaths from "vite-tsconfig-paths";

export default defineConfig({
  plugins: [tsconfigPaths()],
  build: {
    target: "esnext",
  },
  // This is added as a temporary fix for the `process is not defined` issue appearing in one of our dependencies
  define: {
    "process.env": {},
  },
});
