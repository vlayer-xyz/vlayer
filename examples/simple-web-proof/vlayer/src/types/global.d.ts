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

declare module "react-router" {
  export * from "react-router-dom";
}

declare module "react-error-boundary";
declare module "motion/react";
declare module "@heroicons/react/24/outline";

import type * as React from "react";

declare global {
  namespace JSX {
    interface Element extends React.JSX.Element {}
    interface ElementClass extends React.JSX.ElementClass {}
    interface ElementAttributesProperty extends React.JSX.ElementAttributesProperty {}
    interface ElementChildrenAttribute extends React.JSX.ElementChildrenAttribute {}
    interface IntrinsicElements {
      div: React.DetailedHTMLProps<React.HTMLAttributes<HTMLDivElement>, HTMLDivElement>;
      a: React.DetailedHTMLProps<React.AnchorHTMLAttributes<HTMLAnchorElement>, HTMLAnchorElement>;
      b: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
      br: React.DetailedHTMLProps<React.HTMLAttributes<HTMLBRElement>, HTMLBRElement>;
    }
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

/// <reference types="react/jsx-runtime" />
/// <reference types="react" />
/// <reference types="react-dom" />
/// <reference types="react-router-dom" />
/// <reference types="wagmi" />

// Empty module declarations per requirement
declare module "react" {}
declare module "react-dom" {}
declare module "react-router-dom" {}
declare module "wagmi" {}