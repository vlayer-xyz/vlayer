import { privateKeyToAccount } from "viem/accounts";

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

const checkExtensionInstalled = async () => {
  try {
    await chrome.runtime.sendMessage(vlayerPovingExtensionId, {
      message: "ping",
    });
    return true;
  } catch {
    return false;
  }
};

const useTestPrivateKey =
  !import.meta.env.VITE_USE_WINDOW_ETHEREUM_TRANSPORT &&
  Boolean(import.meta.env.VITE_PRIVATE_KEY);

const testPrivateKey = privateKeyToAccount(
  import.meta.env.VITE_PRIVATE_KEY as `0x${string}`,
);

export {
  isMobile,
  isSupportedBrowser,
  checkExtensionInstalled,
  useTestPrivateKey,
  testPrivateKey,
  vlayerPovingExtensionId,
};
