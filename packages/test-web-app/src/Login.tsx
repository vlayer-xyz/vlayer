import React from "react";
import { useNavigate } from "react-router-dom";

export const Login = () => {
  const navigate = useNavigate();
  return (
    <div className="container">
      <button
        onClick={() => {
          navigate("/dashboard");
        }}
      >
        Login
      </button>
    </div>
  );
};
