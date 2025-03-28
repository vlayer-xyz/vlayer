export const createMessageSender = (
  channel: { postMessage: (message: Message) => void },
  messageId: string,
) => {
  return {
    sendMessage: (message: Message) => {
      channel.postMessage({
        type: messageId,
        payload: message,
      });
    },
  };
};

export const createMessageReceiver = (channel: { onMessage: (message: Message) => void }, messageId: string) => {
  return {
    onMessage: (message: Message) => {
      channel.onMessage(message);
    },
  };