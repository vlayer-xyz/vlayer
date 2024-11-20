import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Source, SourceLegacy } from "./Source";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Target from "./Target";
import { LoginMock } from "./LoginMock";

import "./main.css";

const router = createBrowserRouter([
  {
    path: "/source",
    element: <Source />,
  },
  {
    path: "/source-legacy",
    element: <SourceLegacy />,
  },
  {
    path: "/target",
    element: <Target />,
  },
  {
    path: "/login",
    element: <LoginMock />,
  },
]);

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
);
