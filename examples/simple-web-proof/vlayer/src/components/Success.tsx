import { Link } from "react-router";

export const Success = ({ tx, handle }: { tx: string; handle: string }) => {
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
        @{handle} was minted to {tx.slice(0, 6)}...{tx.slice(-4)}
      </p>
      <div className="mt-7 flex justify-center">
        <Link to="/" id="nextButton">
          Start again
        </Link>
      </div>
    </>
  );
};
