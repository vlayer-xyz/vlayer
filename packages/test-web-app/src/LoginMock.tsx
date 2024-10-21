import React from "react";

export const LoginMock = () => {
  return (
    <button
      data-testid="login-button"
      onClick={() => {
        console.log("clicked");
        window.location.href = "/target";
      }}
    >
      Login
    </button>
  );
};
