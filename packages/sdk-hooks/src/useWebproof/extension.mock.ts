import {
  MessageFromExtensionType,
  type MessageFromExtension,
  type PresentationJSON,
  type WebProofProvider,
} from "@vlayer/sdk";

export class MockExtensionWebProofProvider implements WebProofProvider {
  private listeners: Partial<
    Record<
      MessageFromExtensionType,
      ((
        args: Extract<MessageFromExtension, { type: MessageFromExtensionType }>,
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

  public notifyZkProvingStatus(): void {}

  public addEventListeners<T extends MessageFromExtensionType>(
    messageType: T,
    listener: (args: Extract<MessageFromExtension, { type: T }>) => void,
  ): void {
    if (!this.listeners[messageType]) {
      this.listeners[messageType] = [];
    }
    this.listeners[messageType].push(
      listener as (args: MessageFromExtension) => void,
    );
  }

  public requestWebProof(): void {
    // Simulate async response
    setTimeout(() => {
      if (this.mockBehavior.shouldSucceed) {
        const mockProofDoneMessage: MessageFromExtension = {
          type: MessageFromExtensionType.ProofDone,
          payload: {
            presentationJson:
              this.mockBehavior.mockProof ||
              ({ mock: "proof" } as unknown as PresentationJSON),
            decodedTranscript: {
              sent: "mock sent",
              recv: "mock recv",
            },
          },
        };
        this.listeners[MessageFromExtensionType.ProofDone]?.forEach(
          (listener) => {
            listener(mockProofDoneMessage);
          },
        );
      } else {
        const mockErrorMessage: MessageFromExtension = {
          type: MessageFromExtensionType.ProofError,
          payload: {
            error: this.mockBehavior.mockError || "Mock error occurred",
          },
        };
        this.listeners[MessageFromExtensionType.ProofError]?.forEach(
          (listener) => listener(mockErrorMessage),
        );
      }
    }, this.mockBehavior.delayMs);
  }
}
