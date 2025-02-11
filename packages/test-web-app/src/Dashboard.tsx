import React from "react";
function Dashboard() {
  return (
    <div className="container">
      <button
        onClick={() => {
          window.location.href = "/profile";
        }}
      >
        Go to profile
      </button>
      <button
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
