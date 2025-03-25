import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Analytics } from "@vercel/analytics/react";
import * as Sentry from "@sentry/react";

import Providers from "./providers";
import Router from "./router";
import "./index.css";

if (import.meta.env.VITE_SENTRY_DSN) {
  Sentry.init({
    dsn: import.meta.env.VITE_SENTRY_DSN,
    integrations: [],
  });
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Providers>
      <Router />
    </Providers>
    <Analytics />
  </StrictMode>,
);
