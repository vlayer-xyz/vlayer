import { Link } from "react-router";

export const SuccessStepPresentational = ({
  tx,
  handle,
  blockExplorer,
}: {
  tx: string;
  handle: string;
  blockExplorer?: string;
}) => {
  return (
    <>
      <p className="text-gray-500">
        @{handle} was minted to{" "}
        <a
          href={`${blockExplorer}/tx/${tx}`}
          target="_blank"
          rel="noreferrer"
          className="text-violet-500 underline"
        >
          {tx.slice(0, 6)}...{tx.slice(-4)}
        </a>
      </p>
      <div className="mt-2 flex justify-center">
        <Link to="/" id="nextButton">
          Start again
        </Link>
      </div>
    </>
  );
};
