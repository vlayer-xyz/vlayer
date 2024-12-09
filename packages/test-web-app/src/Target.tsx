import React, { useEffect } from "react";

function Target() {
  useEffect(() => {
    const getGandalf = async () =>
      await fetch("https://lotr-api.online:3011/regular_json");
    getGandalf().catch(console.error);
  }, []);
  return (
    <div className="container">
      <h1> Target </h1>
    </div>
  );
}

function MiddleTarget() {
  return (
    <div className="container">
      <h1> Middle Target </h1>
      <button
        data-testid="go-to-target-button"
        onClick={() => {
          window.location.href = "/target";
        }}
      >
        Login
      </button>
    </div>
  );
}

export { Target, MiddleTarget };
