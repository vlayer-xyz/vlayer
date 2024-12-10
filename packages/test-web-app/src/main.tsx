import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Dapp, DappNewWay } from "./Dapp";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { Profile } from "./Profile";
import { Dashboard } from "./Dashboard";
import { Login } from "./Login";
import "./main.css";
import Email from "./Email";

console.log("Dapp", Dapp);

const router = createBrowserRouter([
  // dapp is the app developer used and launched using the sdk
  {
    path: "/dapp",
    element: <Dapp />,
  },
  {
    path: "/dapp-new-way",
    element: <DappNewWay />,
  },
  // profile is route representing place where user is authenticated
  // and therefore has access to data we gonna prove
  {
    path: "/profile",
    element: <Profile />,
  },
  // dashbord is where user is redirected after successful login
  {
    path: "/dashboard",
    element: <Dashboard />,
  },
  // login is route where user is redirected from dapp (via click on redirect button in extension )
  {
    path: "/login",
    element: <Login />,
  },
  // this is not part of store, we test here zk proving email
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
