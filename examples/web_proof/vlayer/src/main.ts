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
    <button id="prove">Request webproof of twitter account</button>
    <button id="vprove" style="margin-top: 10px">Call vlayer prover</button>
    <button id="vverify" style="margin-top: 10px">Call vlayer verifier</button>

  </div>
`;

const twitterProofButton = document.querySelector<HTMLButtonElement>("#prove")!;
const vproveButton = document.querySelector<HTMLButtonElement>("#vprove")!;
setupRequestProveButton(twitterProofButton);
setupVProverButton(vproveButton);
setupVerifyButton(document.querySelector<HTMLButtonElement>("#vverify")!);
