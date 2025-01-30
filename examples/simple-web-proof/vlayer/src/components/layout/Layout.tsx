// Layout.js
import { Outlet } from "react-router";
import { Modal } from "./Modal";

export const Layout = () => {
  return (
    <Modal>
      <Outlet />
    </Modal>
  );
};
