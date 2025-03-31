import { injected } from "wagmi/connectors";
import { useConnect } from "wagmi";

export const ConnectWallet = () => {
  const { connect } = useConnect();

  return (
    <div className="flex flex-col items-center justify-center">
      <a
        onClick={() => connect({ connector: injected() })}
        className="text-violet-500 hover:text-violet-600 font-bold"
      >
        Click here to connect your wallet
      </a>
    </div>
  );
};
