import { createExtensionWebProofProvider } from "@vlayer/sdk/web_proof";
import { useMemo } from "react";

export const useWebProofProvider = () =>
  useMemo(() => createExtensionWebProofProvider(), []);
