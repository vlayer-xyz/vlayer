// Layout.js
import { Outlet } from "react-router";
import { Modal } from "../components/Modal";
import { ProgressBar } from "../components/ProgressBar";

export const Layout = () => {
  return (
    <Modal>
      <ProgressBar />
      <Outlet />
    </Modal>
  );
};
