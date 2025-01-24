import { useSearchParams } from "react-router";
import { useAccount } from "wagmi";
import { Modal } from "../components/Modal";
import { Success } from "../components/Success";

export const SuccessContainer = () => {
  const { chain } = useAccount();
  const [searchParams] = useSearchParams();
  const tx = searchParams.get("tx") || "";
  const handle = searchParams.get("handle") || "";

  return (
    <Modal>
      <Success
        tx={tx}
        handle={handle}
        blockExplorer={chain?.blockExplorers?.default?.url}
      />
    </Modal>
  );
};
