import browser from "webextension-polyfill";

export enum StorageKeys {
  // config passed from SDK, should survive extension close/reopen
  webProverSessionConfig = "webProverSessionConfig",
  // proof computed by the extension, once proof is ready it should survive extension close/reopen
  // other tlsn related data is ephemeral as we have to wait for the proof to be computed in request-response flow
  tlsnProof = "tlsn.proof",
  // browsing history, should survive extension close/reopen
  browsingHistory = "browsingHistory",
  // zk proving status, it will be cleared just after its added
  zkProvingStatus = "zkProvingStatus",
}

export const provingSessionStorageConfig: {
  storage: browser.Storage.StorageArea;
  storageKeys: typeof StorageKeys;
} = {
  storage: browser.storage.session,
  storageKeys: StorageKeys,
};
