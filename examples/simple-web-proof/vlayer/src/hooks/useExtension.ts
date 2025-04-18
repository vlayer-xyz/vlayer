import { useState, useEffect } from "react";
import {
  isMobile,
  isSupportedBrowser,
  checkExtensionInstalled,
} from "../utils";
import { UseExtensionError } from "../errors";
export const useExtension = () => {
  const [hasExtensionInstalled, setHasExtensionInstalled] = useState(false);
  // Using error state to throw error in useEffect because ErrorBoundary does not catch errors from async functions like isExtensionReady
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
    if (!isInstalled) {
      setHasExtensionInstalled(false);
      return;
    }

    setHasExtensionInstalled(true);
  };

  useEffect(() => {
    isExtensionReady().catch(setError);
    const interval = setInterval(
      () => void isExtensionReady().catch(setError),
      5000,
    );
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    if (error) {
      throw new UseExtensionError(error);
    }
  }, [error]);

  return { hasExtensionInstalled };
};
