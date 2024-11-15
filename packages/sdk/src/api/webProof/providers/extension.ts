import {
  type WebProofProvider,
  type WebProofProviderSetup,
  type WebProofSetupInput,
} from "../../lib/types/webProofProvider";

import {
  ExtensionAction,
  type ExtensionMessage,
  ExtensionMessageType,
  type MessageToExtension,
  WebProof,
  ZkProvingStatus,
} from "../../../web-proof-commons";

// NOTE @types/chrome and webextension-polyfill work only in the extension context
// and looks that there is no community driven package providing typings for chrome.runtime
// or polyfill logic for the browser APIs available in the browser context
// we intentionally use chrome here instead of browser as we support only chrome for now
// and there could be some differences in the API between browsers

declare const chrome: {
  runtime: {
    sendMessage: (
      extensionId: string | undefined,
      message: MessageToExtension,
    ) => void;
    connect: (extensionId: string) => {
      onMessage: {
        addListener: (message: unknown) => void;
      };
      postMessage: (message: MessageToExtension) => void;
    };
  };
};

// this id is fixed in the extension by the key in manifest.json
const EXTENSION_ID = "jbchhcgphfokabmfacnkafoeeeppjmpl";

class ExtensionWebProofProvider implements WebProofProvider {
  private port: ReturnType<typeof chrome.runtime.connect> | null = null;
  constructor(
    private notaryUrl: string,
    private wsProxyUrl: string,
  ) {}

  public notifyZkProvingStatus(status: ZkProvingStatus) {
    if (typeof chrome !== "undefined") {
      this.connectToExtension().postMessage({
        action: ExtensionAction.NotifyZkProvingStatus,
        payload: { status },
      });
    }
  }
  private connectToExtension() {
    if (!this.port) {
      this.port = chrome.runtime.connect(EXTENSION_ID);
    }
    return this.port;
  }
  public async getWebProof(webProofSetup: WebProofSetupInput) {
    return new Promise<WebProof>((resolve, reject) => {
      this.connectToExtension().onMessage.addListener(
        (message: ExtensionMessage) => {
          if (message.type === ExtensionMessageType.ProofDone) {
            resolve(message.proof);
          }
          if (message.type === ExtensionMessageType.ProofError) {
            reject(new Error(message.error));
          }
        },
      );

      this.connectToExtension().postMessage({
        action: ExtensionAction.RequestWebProof,
        payload: {
          notaryUrl: this.notaryUrl,
          wsProxyUrl: this.wsProxyUrl,
          logoUrl: webProofSetup.logoUrl,
          steps: webProofSetup.steps,
        },
      });
    });
  }
}
export const createExtensionWebProofProvider = ({
  notaryUrl = "https://notary.pse.dev/v0.1.0-alpha.5/",
  wsProxyUrl = "wss://notary.pse.dev/proxy",
}: WebProofProviderSetup = {}): WebProofProvider => {
  return new ExtensionWebProofProvider(notaryUrl, wsProxyUrl);
};
