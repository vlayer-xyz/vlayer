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
      <button
        onClick={() => {
          fetch("https://lotr-api.online:3011/update_resource", {
            method: "PUT",
            body: JSON.stringify({ name: "John Doe" }),
          }).catch((err) => {
            console.error("Update resource error", err);
          });
        }}
      >
        Update resource
      </button>
    </div>
  );
}

export { Dashboard };
