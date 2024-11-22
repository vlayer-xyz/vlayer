import {
  type WebProofProvider,
  type WebProofProviderSetup,
  type WebProofRequestInput,
} from "../../lib/types/webProofProvider";

import {
  ExtensionAction,
  type ExtensionMessage,
  ExtensionMessageType,
  WebProof,
  ZkProvingStatus,
} from "../../../web-proof-commons";

// Chrome runtime API types for browser context

type ExtensionVersion = string | null;
export const EXTENSION_ID = "jbchhcgphfokabmfacnkafoeeeppjmpl";

class ExtensionWebProofProvider implements WebProofProvider {
  private port: ReturnType<typeof chrome.runtime.connect> | null = null;
  private extensionVersion?: ExtensionVersion;
  private listeners: Partial<
    Record<
      ExtensionMessageType,
      ((
        args: Extract<ExtensionMessage, { type: ExtensionMessageType }>,
      ) => void)[]
    >
  > = {};

  constructor(
    private notaryUrl: string,
    private wsProxyUrl: string,
  ) {}

  public notifyZkProvingStatus(status: ZkProvingStatus) {
    if (typeof chrome !== "undefined") {
      const version = this.extensionVersion;
      if (version) {
        chrome.runtime.sendMessage(EXTENSION_ID, {
          action: ExtensionAction.NotifyZkProvingStatus,
          payload: { status },
        });
      }
    }
  }

  public getExtensionVersion() {
    return new Promise<ExtensionVersion>((resolve) => {
      if (this.extensionVersion !== undefined) {
        resolve(this.extensionVersion);
        return;
      }
      try {
        const port = this.connectToExtension();
        port.postMessage({
          action: ExtensionAction.RequestVersion,
        });

        // It should reply within 1 second
        const timeout = setTimeout(() => {
          this.extensionVersion = null;
          resolve(null);
        }, 1000);

        port.onMessage.addListener((message: ExtensionMessage) => {
          if (message.type === ExtensionMessageType.Version) {
            this.extensionVersion = message.payload.version;
            clearTimeout(timeout);
            resolve(message.payload.version);
          }
        });
      } catch (err) {
        console.error("Error getting extension version", err);
        resolve(null);
      }
    });
  }

  private connectToExtension() {
    if (!this.port) {
      this.port = chrome.runtime.connect(EXTENSION_ID);
      this.port.onMessage.addListener((message: ExtensionMessage) => {
        if (message.type === ExtensionMessageType.ProofDone) {
          this.listeners[ExtensionMessageType.ProofDone]?.forEach((cb) => {
            cb(message);
          });
        }
        if (message.type === ExtensionMessageType.ProofError) {
          this.listeners[ExtensionMessageType.ProofError]?.forEach((cb) => {
            cb(message);
          });
        }
      });
    }
    return this.port;
  }

  public addEventListeners<T extends ExtensionMessageType>(
    messageType: T,
    listener: (args: Extract<ExtensionMessage, { type: T }>) => void,
  ) {
    this.connectToExtension();
    if (!this.listeners[messageType]) {
      this.listeners[messageType] = [];
    }
    this.listeners[messageType]!.push(
      listener as (args: ExtensionMessage) => void,
    );
  }

  public requestWebProof(webProofRequest: WebProofRequestInput) {
    this.connectToExtension().postMessage({
      action: ExtensionAction.RequestWebProof,
      payload: {
        notaryUrl: this.notaryUrl,
        wsProxyUrl: this.wsProxyUrl,
        logoUrl: webProofRequest.logoUrl,
        steps: webProofRequest.steps,
      },
    });
  }

  public async getWebProof(
    webProofRequest: WebProofRequestInput,
  ): Promise<WebProof> {
    return new Promise<WebProof>((resolve, reject) => {
      chrome.runtime.sendMessage(EXTENSION_ID, {
        action: ExtensionAction.RequestWebProof,
        payload: {
          notaryUrl: this.notaryUrl,
          wsProxyUrl: this.wsProxyUrl,
          logoUrl: webProofRequest.logoUrl,
          steps: webProofRequest.steps,
        },
      });

      this.connectToExtension().onMessage.addListener(
        (message: ExtensionMessage) => {
          if (message.type === ExtensionMessageType.ProofDone) {
            resolve(message.payload.proof);
          }
          if (message.type === ExtensionMessageType.ProofError) {
            reject(new Error(message.payload.error));
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
  const provider = new ExtensionWebProofProvider(notaryUrl, wsProxyUrl);
  void provider.getExtensionVersion();
  return provider;
};
