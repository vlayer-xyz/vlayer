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
      chrome.runtime.sendMessage(EXTENSION_ID, {
        action: ExtensionAction.NotifyZkProvingStatus,
        payload: { status },
      });
    }
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

  public requestWebProof(webProofSetup: WebProofSetupInput) {
    this.connectToExtension().postMessage({
      action: ExtensionAction.RequestWebProof,
      payload: {
        notaryUrl: this.notaryUrl,
        wsProxyUrl: this.wsProxyUrl,
        logoUrl: webProofSetup.logoUrl,
        steps: webProofSetup.steps,
      },
    });
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
  return new ExtensionWebProofProvider(notaryUrl, wsProxyUrl);
};
