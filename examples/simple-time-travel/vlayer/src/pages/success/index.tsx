import { useSearchParams } from "react-router";
import { shortenAndFormatHash } from "../../shared/lib/utils";
import { useAccount } from "wagmi";
export const SuccessPage = () => {
  const account = useAccount();
  const [searchParams] = useSearchParams();
  const txHash = searchParams.get("txHash");

  return (
    <>
      <div className="mt-5 flex justify-center text-slate-900">
        <div>
          Here is your NFT:{" "}
          <a
            href={`${account.chain?.blockExplorers?.default.url}/tx/${txHash}`}
            className="text-blue-700 text-center text-block font-bold"
          >
            {shortenAndFormatHash(txHash)}
          </a>
        </div>
      </div>
    </>
  );
};
