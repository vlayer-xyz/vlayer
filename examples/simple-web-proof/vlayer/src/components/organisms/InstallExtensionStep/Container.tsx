import { useExtension } from "../../../hooks/useExtension";
import { InstallExtensionPresentational } from "./Presentationa";
import { useNavigate } from "react-router";
import { useEffect } from "react";

export const InstallExtension = () => {
  const { hasExtensionInstalled } = useExtension();

  const navigate = useNavigate();

  useEffect(() => {
    if (hasExtensionInstalled) {
      void navigate("/start-proving");
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hasExtensionInstalled]);

  return <InstallExtensionPresentational />;
};
