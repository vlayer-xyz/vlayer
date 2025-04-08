import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  // This is added as a temporary fix for the `process is not defined` issue
  // (https://github.com/reown-com/appkit/issues/3926)
  // appearing in our dependency: reown/appkit
  define: {
    "process.env": {},
  },
});
