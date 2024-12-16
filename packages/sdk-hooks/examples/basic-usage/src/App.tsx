import { VlayerProvider } from "@vlayer/react";
function App() {
  return (
    <VlayerProvider
      config={{
        chainName: "sepolia",
        proverUrl: "https://prover.vlayer.xyz",
        jsonRpcUrl: "https://sepolia.infura.io/v3/YOUR_INFURA_PROJECT_ID",
        privateKey: "0xYOUR_PRIVATE_KEY",
        notaryUrl: "https://notary.vlayer.xyz",
        wsProxyUrl: "wss://wsproxy.vlayer.xyz",
      }}
    >
      <div>Here will come the UI</div>
    </VlayerProvider>
  );
}

export default App;
