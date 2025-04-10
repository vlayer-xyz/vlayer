import React from "react";
import ReactDOM from "react-dom/client";
import "@radix-ui/themes/styles.css";
import { SidePanel } from "components/pages/SidePanel";
import { initSentry } from "src/helpers/sentry";

initSentry();

ReactDOM.createRoot(document.body).render(
  <React.StrictMode>
    <SidePanel />
  </React.StrictMode>,
);
