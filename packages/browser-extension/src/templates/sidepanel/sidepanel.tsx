import React from "react";
import ReactDOM from "react-dom/client";
// import SidePanel from "../../pages/SidePanel";
import "@radix-ui/themes/styles.css";
import { NewSidePanel } from "components/pages/NewSidePanel";

ReactDOM.createRoot(document.body).render(
  <React.StrictMode>
    <NewSidePanel />
  </React.StrictMode>,
);
