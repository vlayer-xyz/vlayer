// Layout.js
import { Outlet } from "react-router";
import { Modal } from "./Modal";
import { ProgressBar } from "../molecules/ProgressBar";

export const Layout = () => {
  return (
    <Modal>
      <ProgressBar />
      <Outlet />
    </Modal>
  );
};
