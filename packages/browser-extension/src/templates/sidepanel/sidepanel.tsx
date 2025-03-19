import React from "react";
import ReactDOM from "react-dom/client";
import "@radix-ui/themes/styles.css";
import * as Sentry from "@sentry/react";
import { SidePanel } from "components/pages/SidePanel";

if (import.meta.env.VITE_SENTRY_DSN) {
  Sentry.init({
    dsn: import.meta.env.VITE_SENTRY_DSN,
    integrations: [],
    release: chrome.runtime.getManifest().version_name,
  });
}

ReactDOM.createRoot(document.body).render(
  <React.StrictMode>
    <SidePanel />
  </React.StrictMode>,
);
