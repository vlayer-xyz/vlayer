import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  resolve:
    process.env.VLAYER_ENV === "dev" ? { conditions: ["development"] } : {},
});
