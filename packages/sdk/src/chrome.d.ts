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
