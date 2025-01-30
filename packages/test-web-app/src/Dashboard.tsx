import React from "react";
function Dashboard() {
  return (
    <div className="container">
      <button
        data-testid="go-to-profile-button"
        onClick={() => {
          window.location.href = "/profile";
        }}
      >
        Go to profile
      </button>
      <button
        data-testid="go-to-profile-failed-auth-button"
        onClick={() => {
          window.location.href = "/profile-failed-auth";
        }}
      >
        Go to profile failed auth
      </button>
    </div>
  );
}

export { Dashboard };
