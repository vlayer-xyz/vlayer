import { modalContext } from "../components/Modal";
import { useContext } from "react";

export const useModal = () => {
  const { showModal, closeModal } = useContext(modalContext);
  return { showModal, closeModal };
};
