import "./style.css";
import {
  setupRequestProveButton,
  setupVerifyButton,
  setupVProverButton,
} from "./prove";
document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div class="container">
    <div class="logoContainer">
      <img src="vlayer_logo.svg" />
    </div>
    <button data-testid="request-webproof-button" id="prove">Request webproof of twitter account</button>
    <button data-testid="vprove-button" id="vprove" style="margin-top: 10px">Call vlayer prover</button>
    <button data-testid="verify-button" id="verify" style="margin-top: 10px">Call vlayer verifier</button>

  </div>
`;

const twitterProofButton = document.querySelector<HTMLButtonElement>("#prove")!;
const vproveButton = document.querySelector<HTMLButtonElement>("#vprove")!;
setupRequestProveButton(twitterProofButton);
setupVProverButton(vproveButton);
setupVerifyButton(document.querySelector<HTMLButtonElement>("#verify")!);
