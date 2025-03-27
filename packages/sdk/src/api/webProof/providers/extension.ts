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
  type WebProofStep,
  ZkProvingStatus,
  assertUrl,
  assertUrlPattern,
  type RedactionConfig,
  RedactionItemsArray,
  type MessageToExtension,
} from "@vlayer/web-proof-commons";

import debug from "debug";

const log = debug("vlayer:WebProof:provider");
const EXTENSION_ID = "jbchhcgphfokabmfacnkafoeeeppjmpl";

declare let chrome: {
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
      onDisconnect: {
        addListener: (callback: () => void) => void;
      };
    };
  };
};

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
    private jwtToken: string | null,
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
        log("Cant send message", "look that extension is not installed ");
      }
    }
  }

  private connectToExtension() {
    if (!this.port) {
      this.port = chrome.runtime.connect(EXTENSION_ID);
      this.port.onDisconnect.addListener(() => {
        this.port = null;
        this.connectToExtension();
      });
      this.port.onMessage.addListener((message: ExtensionMessage) => {
        this.listeners[message.type]?.forEach((cb) => cb(message));
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
    this.listeners[messageType].push(
      listener as (args: ExtensionMessage) => void,
    );
  }

  public closeSidePanel() {
    const port = this.connectToExtension();
    port.postMessage({
      action: ExtensionAction.CloseSidePanel,
    });
  }

  public openSidePanel() {
    this.connectToExtension().postMessage({
      action: ExtensionAction.OpenSidePanel,
    });
  }

  public requestWebProof(webProofRequest: WebProofRequestInput) {
    validateWebProofRequest(webProofRequest);
    this.connectToExtension().postMessage({
      action: ExtensionAction.RequestWebProof,
      payload: {
        notaryUrl: this.notaryUrl,
        wsProxyUrl: this.wsProxyUrl,
        jwtToken: this.jwtToken,
        logoUrl: webProofRequest.logoUrl,
        steps: webProofRequest.steps,
      },
    });
  }
}

const validateSteps = (steps: WebProofStep[]) => {
  steps.forEach((step) => {
    if (step.step === EXTENSION_STEP.startPage) {
      assertUrl(step.url);
    } else {
      assertUrlPattern(step.url);
    }
    if (step.step === EXTENSION_STEP.notarize) {
      validateRedaction(step.redact ?? []);
    }
  });
};

const validateRedaction = (redaction: RedactionConfig) => {
  RedactionItemsArray.parse(redaction);
};

export const validateWebProofRequest = (
  webProofRequest: WebProofRequestInput,
) => {
  validateSteps(webProofRequest.steps);
};

export const createExtensionWebProofProvider = ({
  notaryUrl = "https://test-notary.vlayer.xyz",
  wsProxyUrl = "wss://test-wsproxy.vlayer.xyz",
  jwtToken = null,
}: WebProofProviderSetup = {}): WebProofProvider => {
  return new ExtensionWebProofProvider(notaryUrl, wsProxyUrl, jwtToken);
};
