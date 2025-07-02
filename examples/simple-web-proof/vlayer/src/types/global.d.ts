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

declare module "react" {
  export type ReactNode = any;
  export interface ComponentType<P = any> {
    (props: P): any;
  }
  export interface FC<P = {}> extends ComponentType<P> {}
  export interface FunctionComponent<P = {}> extends ComponentType<P> {}
  export function createElement(...args: any[]): any;
  export function useState<T>(initial: T): [T, (newVal: T) => void];
  export function useEffect(...args: any[]): void;
  export namespace JSX {
    interface Element {}
    interface IntrinsicAttributes {}
    interface IntrinsicElements {
      [elemName: string]: any;
    }
  }
}

declare module "react-dom";
declare module "react/jsx-runtime";

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