import type { PresentationJSON, ZkProvingStatus } from "@vlayer/sdk";

import {
  ExtensionMessageType,
  type ExtensionMessage,
  type WebProofProvider,
} from "@vlayer/sdk";

export class MockExtensionWebProofProvider implements WebProofProvider {
  private listeners: Partial<
    Record<
      ExtensionMessageType,
      ((
        args: Extract<ExtensionMessage, { type: ExtensionMessageType }>,
      ) => void)[]
    >
  > = {};

  constructor(
    private mockBehavior: {
      shouldSucceed?: boolean;
      delayMs?: number;
      mockProof?: PresentationJSON;
      mockError?: string;
    } = { shouldSucceed: true, delayMs: 100 },
  ) {}

  public notifyZkProvingStatus(status: ZkProvingStatus): void {
    console.log("Mock: ZK proving status notification", status);
  }

  public addEventListeners<T extends ExtensionMessageType>(
    messageType: T,
    listener: (args: Extract<ExtensionMessage, { type: T }>) => void,
  ): void {
    if (!this.listeners[messageType]) {
      this.listeners[messageType] = [];
    }
    this.listeners[messageType].push(
      listener as (args: ExtensionMessage) => void,
    );
  }

  public requestWebProof(): void {
    // Simulate async response
    setTimeout(() => {
      if (this.mockBehavior.shouldSucceed) {
        const mockProofDoneMessage: ExtensionMessage = {
          type: ExtensionMessageType.ProofDone,
          payload: {
            presentationJSON:
              this.mockBehavior.mockProof ||
              ({ mock: "proof" } as unknown as PresentationJSON),
            decodedTranscript: {
              sent: "mock sent",
              recv: "mock recv",
            },
          },
        };
        this.listeners[ExtensionMessageType.ProofDone]?.forEach((listener) => {
          console.log("Mock: ProofDone message", mockProofDoneMessage);
          listener(mockProofDoneMessage);
        });
      } else {
        const mockErrorMessage: ExtensionMessage = {
          type: ExtensionMessageType.ProofError,
          payload: {
            error: this.mockBehavior.mockError || "Mock error occurred",
          },
        };
        this.listeners[ExtensionMessageType.ProofError]?.forEach((listener) =>
          listener(mockErrorMessage),
        );
      }
    }, this.mockBehavior.delayMs);
  }

  public async getWebProof(): Promise<{
    presentationJSON: PresentationJSON;
    decodedTranscript: {
      sent: string;
      recv: string;
    };
  }> {
    await new Promise((resolve) =>
      setTimeout(resolve, this.mockBehavior.delayMs),
    );

    if (this.mockBehavior.shouldSucceed) {
      return {
        presentationJSON:
          this.mockBehavior.mockProof ||
          ({ mock: "proof" } as unknown as PresentationJSON),
        decodedTranscript: {
          sent: "mock sent",
          recv: "mock recv",
        },
      };
    } else {
      throw new Error(this.mockBehavior.mockError || "Mock error occurred");
    }
  }
}
