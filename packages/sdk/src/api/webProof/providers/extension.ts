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

// Chrome runtime API types for browser context
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

const EXTENSION_ID = "jbchhcgphfokabmfacnkafoeeeppjmpl";

class ExtensionWebProofProvider implements WebProofProvider {
  private port: ReturnType<typeof chrome.runtime.connect> | null = null;

  private listeners = {
    [ExtensionMessageType.ProofDone]: [] as ((proof: WebProof) => void)[],
    [ExtensionMessageType.ProofError]: [] as ((error: Error) => void)[],
  };

  constructor(
    private notaryUrl: string,
    private wsProxyUrl: string,
  ) {
    this.connectToExtension().onMessage.addListener(
      (message: ExtensionMessage) => {
        if (message.type === ExtensionMessageType.ProofDone) {
          this.listeners[ExtensionMessageType.ProofDone].forEach((cb) => {
            cb(message.proof);
          });
        }
        if (message.type === ExtensionMessageType.ProofError) {
          this.listeners[ExtensionMessageType.ProofError].forEach((cb) => {
            cb(new Error(message.error));
          });
        }
      },
    );
  }

  public notifyZkProvingStatus(status: ZkProvingStatus) {
    if (typeof chrome !== "undefined") {
      chrome.runtime.sendMessage(EXTENSION_ID, {
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

  onWebProofDone(callback: (proof: WebProof) => void) {
    this.listeners[ExtensionMessageType.ProofDone].push(callback);
  }

  onWebProofError(callback: (error: Error) => void) {
    this.listeners[ExtensionMessageType.ProofError].push(callback);
  }

  public async getWebProof(
    webProofSetup: WebProofSetupInput,
  ): Promise<WebProof> {
    return new Promise<WebProof>((resolve, reject) => {
      chrome.runtime.sendMessage(EXTENSION_ID, {
        action: ExtensionAction.RequestWebProof,
        payload: {
          notaryUrl: this.notaryUrl,
          wsProxyUrl: this.wsProxyUrl,
          logoUrl: webProofSetup.logoUrl,
          steps: webProofSetup.steps,
        },
      });

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
    });
  }
}

export const createExtensionWebProofProvider = ({
  notaryUrl = "https://notary.pse.dev/v0.1.0-alpha.5/",
  wsProxyUrl = "wss://notary.pse.dev/proxy",
}: WebProofProviderSetup = {}): WebProofProvider => {
  return new ExtensionWebProofProvider(notaryUrl, wsProxyUrl);
};
