import { useSearchParams } from "react-router";
import { useAccount } from "wagmi";
import { SuccessStepPresentational } from "./Presentational";

export const SuccessStep = () => {
  const { chain } = useAccount();
  const [searchParams] = useSearchParams();
  const tx = searchParams.get("tx") || "";
  const handle = searchParams.get("handle") || "";

  return (
    <SuccessStepPresentational
      tx={tx}
      handle={handle}
      blockExplorer={chain?.blockExplorers?.default?.url}
    />
  );
};
