import { Link } from "react-router";
import { Modal } from "./Modal";

export const WelcomeScreen = () => {
  return (
    <Modal>
      <div className="flex justify-center mb-4">
        <img
          src="/logo-evangelist.svg"
          alt="NFT Icon"
          className="w-[282px] h-[100px]"
        />
      </div>
      <p className="desc">Verify list of followers of a specific X account.</p>
      <div className="mt-5 flex justify-center">
        <Link to="/start-proving" id="nextButton">
          Start
        </Link>
      </div>
    </Modal>
  );
};
