import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Analytics } from "@vercel/analytics/react";
import "./index.css";
import App from "./App.tsx";
import * as Sentry from "@sentry/react";

if (import.meta.env.VITE_SENTRY_EMAIL_PROOF) {
  Sentry.init({
    dsn: import.meta.env.VITE_SENTRY_EMAIL_PROOF,
    integrations: [],
  });
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
    <Analytics />
  </StrictMode>,
);
