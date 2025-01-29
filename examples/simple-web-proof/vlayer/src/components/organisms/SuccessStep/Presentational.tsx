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
      <div className="flex justify-center">
        <img
          src="/success-illustration.svg"
          alt="Success Icon"
          className="w-[282px] h-[155px]"
        />
      </div>
      <h3 className="mt-7 header">Success</h3>
      <p className="py-4 text-gray-500">
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
      <div className="mt-7 flex justify-center">
        <Link to="/" id="nextButton">
          Start again
        </Link>
      </div>
    </>
  );
};
