import React from "react";
import { redirect } from "react-router-dom";
import { useNavigate } from "react-router-dom";

export const LoginMock = () => {
  const navigate = useNavigate();

  return (
    <button
      data-testid="login-button"
      onClick={() => {
        console.log("clicked");
        window.location.href = "/target";
        // navigate("/target");
      }}
    >
      Login
    </button>
  );
};
