import React from "react";

export const LoginMock = () => {
  return (
    <div className="container">
      <button
        data-testid="login-button"
        onClick={() => {
          window.location.href = "/target";
        }}
      >
        Login
      </button>
    </div>
  );
};
