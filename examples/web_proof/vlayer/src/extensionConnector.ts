import gpProof from "../tls_gp_proof.json";
export const extensionConnector: {
  port?: chrome.runtime.Port;
  windowId?: number;
  connect: () => void;
  tlsproof?: object;
  zkproof?: object;
  returnValue?: unknown;
} = {
  port: undefined,
  tlsproof: gpProof,

  connect: function () {
    this.port = chrome.runtime.connect(import.meta.env.VITE_EXTENSION_ID);
    console.log("Connected to extension", this.port);
    this.port.onMessage.addListener((message) => {
      console.log("Message from extension", message);
      if (message.type === "proof_done") {
        this.tlsproof = message.proof;
      }
    });
    chrome.tabs.onActivated.addListener((activeInfo) => {
      this.windowId = activeInfo.windowId;
    });
  },
};
