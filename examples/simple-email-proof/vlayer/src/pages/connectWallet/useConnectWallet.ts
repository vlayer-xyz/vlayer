import { useEffect } from "react";
import { injected, useAccount, useConnect } from "wagmi";
import { useNavigate } from "react-router";

export const useConnectWallet = () => {
  const { connect } = useConnect();
  const { address } = useAccount();
  const navigate = useNavigate();

  useEffect(() => {
    if (address) {
      void navigate("/send-email");
    }
  }, [address, navigate]);

  const connectWallet = () => {
    connect({
      connector: injected(),
    });
  };

  return { connectWallet };
};
