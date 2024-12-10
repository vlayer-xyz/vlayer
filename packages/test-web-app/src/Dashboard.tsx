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
    </div>
  );
}

export { Dashboard };
