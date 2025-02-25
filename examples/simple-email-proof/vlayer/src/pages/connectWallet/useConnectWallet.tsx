import { useEffect } from "react";
import { injected, useAccount, useConnect } from "wagmi";
import { useNavigate } from "react-router";

export const useConnectWallet = () => {
  const { connect } = useConnect();
  const { address } = useAccount();
  const navigate = useNavigate();

  useEffect(() => {
    if (address) {
      navigate("/send-email");
    }
  }, [address]);

  const connectWallet = () => {
    connect({
      connector: injected(),
    });
  };

  return { connectWallet };
};
