import { useExtension } from "../../../hooks/useExtension";
import { InstallExtensionPresentational } from "./Presentationa";
import { useNavigate } from "react-router";
import { useEffect } from "react";

export const InstallExtension = () => {
  const { hasExtensionInstalled } = useExtension();

  const navigate = useNavigate();

  useEffect(() => {
    if (hasExtensionInstalled) {
      navigate("/start-proving");
    }
  }, [hasExtensionInstalled]);

  return <InstallExtensionPresentational />;
};
