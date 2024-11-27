import { Button } from "react-daisyui";

const PROVER_ADDRESS = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
import unconditionalProver from "../../../../contracts/fixtures/out/UnconditionalProver.sol/UnconditionalProver";
import { expectUrl, notarize, startPage } from "@vlayer/sdk/web_proof";
import { foundry } from "viem/chains";

const config = {
  proverCallCommitment: {
    address: PROVER_ADDRESS,
    proverAbi: unconditionalProver.abi,
    chainId: foundry.id,
    functionName: "web_proof",
    commitmentArgs: [],
  },
  logoUrl: "",
  steps: [
    startPage("https://demo.tink.com/", "Go to login"),
    startPage("https://demo.tink.com/account-check", "Go to account check"),
    expectUrl(targetUrl, "Logged in and appear at target page"),
    notarize("https://swapi.dev/api/people/1", "GET", "Prove"),
  ],
};

function App() {
  const vlayerFlow = useVlayerFlow();

  return (
    <div className="flex justify-center mt-48">
      <Button color="primary">Request Web Proof</Button>
    </div>
  );
}

export default App;
