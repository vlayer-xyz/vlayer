import { modalContext } from "../layout/Modal";
import { useContext } from "react";

export const useModal = () => {
  const { showModal, closeModal } = useContext(modalContext);
  return { showModal, closeModal };
};
