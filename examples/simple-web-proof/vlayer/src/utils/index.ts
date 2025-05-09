/// <reference types="chrome" />

import { MessageFromExtensionType } from "@vlayer/sdk";
import { z } from "zod";
const vlayerPovingExtensionId = "jbchhcgphfokabmfacnkafoeeeppjmpl";

const isMobile =
  /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
    navigator.userAgent,
  );

const isSupportedBrowser = () => {
  const userAgent = navigator.userAgent.toLowerCase();

  const isChromiumBased = Boolean(
    userAgent.includes("chrome") || userAgent.includes("chromium"),
  );

  return isChromiumBased;
};

const responseSchema = z.object({
  type: z.literal(MessageFromExtensionType.Pong),
});

const checkExtensionInstalled = async () => {
  let response;

  try {
    response = await chrome.runtime.sendMessage<unknown, unknown>(
      vlayerPovingExtensionId,
      {
        message: "ping",
      },
    );
  } catch {
    return false;
  }

  const parsedResponse = responseSchema.safeParse(response);

  if (parsedResponse.success) {
    return true;
  }
  return false;
};

export {
  isMobile,
  isSupportedBrowser,
  checkExtensionInstalled,
  vlayerPovingExtensionId,
};
