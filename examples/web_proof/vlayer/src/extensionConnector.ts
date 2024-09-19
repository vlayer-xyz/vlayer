export const extensionConnector: {
  port?: chrome.runtime.Port;
  connect: () => void;
} = {
  port: undefined,
  connect: function () {
    this.port = chrome.runtime.connect(import.meta.env.VITE_EXTENSION_ID);
    console.log("Connected to extension", this.port);
    this.port.onMessage.addListener((message) => {
      console.log("Message from extension", message);
    });
  },
};
