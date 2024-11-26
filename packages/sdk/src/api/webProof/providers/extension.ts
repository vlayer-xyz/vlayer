import {
  type WebProofProvider,
  type WebProofProviderSetup,
  type WebProofRequestInput,
} from "../../lib/types/webProofProvider";

import {
  EXTENSION_STEP,
  ExtensionAction,
  type ExtensionMessage,
  ExtensionMessageType,
  type MessageToExtension,
  WebProof,
  WebProofStep,
  ZkProvingStatus,
  assertUrl,
  assertUrlPattern,
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
      // Chrome does not provide reliable api to check if given extension is installed
      // what we could do is to use management api but
      // 1) this will need to provided extra permission
      // 2) still is not reliable because this api becomes defined when first extension that uses it is installed
      // so still will need to try catch
      try {
        chrome.runtime.sendMessage(EXTENSION_ID, {
          action: ExtensionAction.NotifyZkProvingStatus,
          payload: { status },
        });
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
      } catch (e) {
        console.log(
          "Cant send message",
          "look that extension is not installed ",
        );
      }
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

  public requestWebProof(webProofRequest: WebProofRequestInput) {
    validateWebProofRequest(webProofRequest);
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

const validateSteps = (steps: WebProofStep[]) => {
  steps.forEach(({ step, url }) => {
    if (step === EXTENSION_STEP.startPage) {
      assertUrl(url);
    } else {
      assertUrlPattern(url);
    }
  });
};

export const validateWebProofRequest = (
  webProofRequest: WebProofRequestInput,
) => {
  validateSteps(webProofRequest.steps);
};

export const createExtensionWebProofProvider = ({
  notaryUrl = "https://notary.pse.dev/v0.1.0-alpha.5/",
  wsProxyUrl = "wss://notary.pse.dev/proxy",
}: WebProofProviderSetup = {}): WebProofProvider => {
  return new ExtensionWebProofProvider(notaryUrl, wsProxyUrl);
};
