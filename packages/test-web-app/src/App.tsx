import {
  createExtensionWebProofProvider,
  expectUrl,
  notarize,
  startPage,
} from "@vlayer/sdk";
import React, { useEffect } from "react";
import { BrowserRouter as Router, Route, Routes, Link } from "react-router-dom";

const requestWebProof = async () => {
  const provider = createExtensionWebProofProvider({});
  await provider.getWebProof({
    //@ts-expect-error this part is not implemented yet in our tlsn flow
    proverCallCommitment: {},
    steps: [
      startPage("http://localhost:5174/swapi", "Go to swapi"), 
      expectUrl("http://localhost:5174/swapi", "Visiting swapi"), 
      notarize("https://swapi.dev/api/people/1", "Notarize Swapi", "Notarize!")
    ],
  });
};

function Home() {
  return (
    <button
      data-testid="request-webproof-button"
      onClick={requestWebProof}
    ></button>
  );
}

function Swapi() {
  useEffect(() => {
    const fetchData = async () => {
      try {
        await fetch("https://swapi.dev/api/people/1");
      } catch (error) {
        console.error("Error fetching data:", error);
      }
    };

    fetchData();
  }, []);

  return <h2>About</h2>;
}

function App() {
  return (
    <Router>
      <nav>
        <ul>
          <li>
            <Link to="/">Home</Link>
          </li>
          <li>
            <Link to="/swapi">Swapi</Link>
          </li>
        </ul>
      </nav>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/swapi" element={<Swapi />} />
      </Routes>
    </Router>
  );
}

export default App;