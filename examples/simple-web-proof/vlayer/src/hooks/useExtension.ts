import { useState, useEffect } from "react";
import {
  isMobile,
  isSupportedBrowser,
  checkExtensionInstalled,
} from "../utils";
export const useExtension = () => {
  const [hasExtensionInstalled, setHasExtensionInstalled] = useState(false);
  const [error, setError] = useState<string | undefined>(undefined);

  const isExtensionReady = async () => {
    if (isMobile) {
      setError("Mobile browsers are not supported");
      setHasExtensionInstalled(false);
      return;
    }
    if (!isSupportedBrowser()) {
      setError("Unsupported browser. Please try Chrome based browsers.");
      setHasExtensionInstalled(false);
      return;
    }

    const isInstalled = await checkExtensionInstalled();
    console.log("is", isInstalled);
    if (!isInstalled) {
      setError("Please install vlayer extension and try again");
      setHasExtensionInstalled(false);
      return;
    }

    setHasExtensionInstalled(true);
  };

  useEffect(() => {
    isExtensionReady();
    const interval = setInterval(isExtensionReady, 5000);
    return () => clearInterval(interval);
  }, []);
  return { hasExtensionInstalled, error };
};
