import React, { useEffect } from "react";

function Target() {
  useEffect(() => {
    const getLukeSkywalker = async () =>
      await fetch("https://swapi.dev/api/people/1");
    getLukeSkywalker().catch(console.error);
  }, []);
  return (
    <>
      <h1> Target </h1>
    </>
  );
}

export default Target;
