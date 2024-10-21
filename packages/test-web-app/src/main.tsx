/* eslint-disable */
import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import Source from "./Source";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Target from "./Target";
import { LoginMock } from "./LoginMock";

const router = createBrowserRouter([
  {
    path: "/source",
    element: <Source />,
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
