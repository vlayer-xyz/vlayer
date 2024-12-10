import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Source, SourceNewWay } from "./Source";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { Target, MiddleTarget } from "./Target";
import { StartPage } from "./StartPage";
import "./main.css";
import Email from "./Email";
const router = createBrowserRouter([
  {
    path: "/source",
    element: <Source />,
  },
  {
    path: "/source-new-way",
    element: <SourceNewWay />,
  },
  {
    path: "/target",
    element: <Target />,
  },
  {
    path: "/middle-target",
    element: <MiddleTarget />,
  },
  {
    path: "/start-page",
    element: <StartPage />,
  },
  {
    path: "/email",
    element: <Email />,
  },
]);

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
);
