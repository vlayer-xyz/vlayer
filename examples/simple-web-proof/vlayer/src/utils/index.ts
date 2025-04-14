/// <reference types="chrome" />
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

export {
  isMobile,
  isSupportedBrowser,
  checkExtensionInstalled,
  vlayerPovingExtensionId,
};
