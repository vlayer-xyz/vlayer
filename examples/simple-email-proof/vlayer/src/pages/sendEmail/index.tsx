import { useAccount } from "wagmi";
import { SendEmail } from "./Presentational";
import { v4 as uuidv4 } from "uuid";

export const SendEmailContainer = () => {
  const { address } = useAccount();
  const emailId = uuidv4();

  const subject = `Mint my domain NFT at address: ${address}`;
  const uniqueEmail = `${emailId}@proving.vlayer.xyz`;

  if (!address) {
    return <div>Connect your wallet to send an email</div>;
  }

  return <SendEmail subject={subject} uniqueEmail={uniqueEmail} />;
};
