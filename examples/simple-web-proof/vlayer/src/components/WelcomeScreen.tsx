import { Link } from "react-router";
import { Modal } from "./Modal";

export const WelcomeScreen = () => {
  return (
    <Modal>
      <div className="flex justify-center mb-4">
        <img
          src="/nft-illustration.svg"
          alt="NFT Icon"
          className="w-[282px] h-[156px]"
        />
      </div>
      <h3 className="header">X NFT</h3>
      <p className="desc">
        Mint an NFT with your X (previosuly Twitter) account. Only owner of
        account can mint NFT for specific handle. This example demonstrates use
        of Web Proofs.
      </p>
      <div className="mt-5 flex justify-center">
        <Link to="/connect-wallet" id="nextButton">
          Start
        </Link>
      </div>
    </Modal>
  );
};
