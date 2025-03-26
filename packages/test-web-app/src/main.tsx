import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Dapp, DappFailedAuth } from "./Dapp";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { Profile, ProfileFailedAuth } from "./Profile";
import { Dashboard } from "./Dashboard";
import { Login } from "./Login";
import "./main.css";
import Email from "./Email";
import { DappProveWeb } from "./DappProveWeb";
import { DappPut } from "./DappPut";
import SdkPlayground from "./SdkPlayground";

const router = createBrowserRouter([
  // dapp is the app developer used and launched using the sdk
  {
    path: "/dapp",
    element: <Dapp />,
  },
  {
    path: "/dapp-prove-web",
    element: <DappProveWeb />,
  },
  {
    path: "/dapp-failed-auth",
    element: <DappFailedAuth />,
  },
  {
    path: "/dapp-put",
    element: <DappPut />,
  },
  // profile is route representing place where user is authenticated
  // and therefore has access to data we gonna prove
  {
    path: "/profile",
    element: <Profile />,
  },
  {
    path: "/profile-failed-auth",
    element: <ProfileFailedAuth />,
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
  {
    path: "/",
    element: <div></div>,
  },
  {
    path: "/sdk-playground",
    element: <SdkPlayground />,
  },
]);

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
);
