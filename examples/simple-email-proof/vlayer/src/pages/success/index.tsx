import { Link, useSearchParams } from "react-router";
import { truncateHashOrAddr } from "../../shared/lib/utils";
export const SuccessContainer = () => {
  const [searchParams] = useSearchParams();
  const txHash = searchParams.get("txHash");
  const domain = searchParams.get("domain");
  const recipient = searchParams.get("recipient");

  return (
    <>
      <div className="desc in">
        Your <b>{domain}</b> NFT was minted to {truncateHashOrAddr(recipient)}
        <br />
        <a
          href={`https://optimism-sepolia.blockscout.com/tx/${txHash}`}
          target="_blank"
          rel="noreferrer"
          className="text-violet-500 font-bold block mt-5"
        >
          View on block explorer
        </a>
      </div>
      <div className="mt-5 flex justify-center">
        <Link to="/" id="nextButton" data-testid="start-page-button">
          Start again
        </Link>
      </div>
    </>
  );
};
