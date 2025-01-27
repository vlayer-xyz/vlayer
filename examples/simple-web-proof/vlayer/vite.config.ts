import { defineConfig } from "vite";
import tsconfigPaths from "vite-tsconfig-paths";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [tsconfigPaths(), react()],
  build: {
    target: "esnext",
  },
  resolve:
    process.env.VLAYER_ENV === "dev" ? { conditions: ["development"] } : {},
});
