import { useLocalStorage } from "@vlayer/extension-hooks";

//default are set to twitter just for now

const defaultRedirectUrl = "https://x.com";
const defaultProofUrl = "https://api.x.com/1.1/account/settings.json";
const defaultBackUrl = "http://localhost:5134";
export const useProofContext = () => {
  const [redirectUrl] = useLocalStorage("redirectUrl", defaultRedirectUrl);
  const [proofUrl] = useLocalStorage("proofUrl", defaultProofUrl);
  const [backUrl] = useLocalStorage("backUrl", defaultBackUrl);
  return {
    redirectUrl,
    proofUrl,
    backUrl,
  };
};
