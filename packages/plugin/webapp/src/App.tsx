import "./App.css";

function App() {
  console.log("Vlayer playground", import.meta.env);
  return (
    <>
      <h1>Vlayer playground </h1>
      <h3>Contracts :</h3>
      <p>Prover : {import.meta.env.VITE_PROVER_ADDRESS}</p>
      <p>Verifier: {import.meta.env.VITE_VERIFIER_ADDRESS}</p>
    </>
  );
}

export default App;
