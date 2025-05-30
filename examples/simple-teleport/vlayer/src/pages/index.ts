export * from "./welcome";
export * from "./showBalance";
export * from "./confirmMint";
export * from "./success";

const envs = import.meta.env;
// @ts-expect-error - window is not typed
window.envs = envs;
