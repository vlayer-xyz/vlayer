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

export default Target;
