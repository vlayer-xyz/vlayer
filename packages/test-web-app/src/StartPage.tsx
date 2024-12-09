import React from "react";
import { useNavigate } from "react-router-dom";

export const StartPage = () => {
  const navigate = useNavigate();
  return (
    <div className="container">
      <button
        data-testid="go-to-middle-target-button"
        onClick={() => {
          navigate("/middle-target");
        }}
      >
        Login with history.pushState
      </button>
      <button
        data-testid="go-to-target-button"
        onClick={() => {
          window.location.href = "/target";
        }}
      >
        Login
      </button>{" "}
    </div>
  );
};
