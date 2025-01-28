// Layout.js
import { Outlet } from "react-router";
import { Modal } from "../components/Modal";
import { ProgressBar } from "../components/ProgressBar";
import { useState } from "react";
import { AnimatePresence } from "motion/react";

export const Layout = () => {
  const [isModalOpen, setIsModalOpen] = useState(true);
  const closeModal = () => setIsModalOpen(false);
  const showModal = () => setIsModalOpen(true);
  return (
    <AnimatePresence>
      {isModalOpen && (
        <Modal>
          <ProgressBar />
          <Outlet context={{ isModalOpen, closeModal, showModal }} />
        </Modal>
      )}
    </AnimatePresence>
  );
};
