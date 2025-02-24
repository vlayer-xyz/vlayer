import { NextButton } from "./ui/NextButton";
import { InputWithCopy } from "./ui/InputWithCopy";

export const SendEmail = () => {
  return (
    <>
      <div className="w-full">
        <InputWithCopy
          label="Subject"
          value="Mint my domain NFT at address: 0x1234...abcd"
        />
        <InputWithCopy label="To" value="nft@vlayer.xyz" />
      </div>
      <NextButton />
    </>
  );
};
