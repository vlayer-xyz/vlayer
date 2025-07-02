declare module "*.module.css" {
  const classes: { [key: string]: string };
  export default classes;
}

declare module "*.css" {
  const classes: { [key: string]: string };
  export default classes;
}

declare module "*.png" {
  const src: string;
  export default src;
}

declare module "react-router";
declare module "react-error-boundary";
declare module "motion/react";
declare module "@heroicons/react/24/outline";

import type * as React from "react";

declare global {
  namespace JSX {
    interface IntrinsicElements extends React.JSX.IntrinsicElements {}
    interface IntrinsicAttributes extends React.JSX.IntrinsicAttributes {}
  }
}

declare module "wagmi";
declare module "viem";
declare module "viem/*";
declare module "@vlayer/react";
declare module "@vlayer/sdk";
declare module "usehooks-ts";
declare module "@tanstack/react-query";
declare module "debug";
declare module "@johanneskares/wallet-mock";
declare module "@playwright/test";
declare module "fs";

/// <reference types="react" />