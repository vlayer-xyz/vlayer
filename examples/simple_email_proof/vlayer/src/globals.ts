declare global {
  interface Window {
    ethereum: { request: () => Promise<unknown> };
  }
}
