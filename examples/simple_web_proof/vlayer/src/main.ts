import "./style.css";
import {
  setupProveWebButton,
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
    <button id="proveweb" style="margin-top: 10px">Request webproof of web page</button>
    <button id="vverify" style="margin-top: 10px">Call vlayer verifier</button>

  </div>
`;

const twitterProofButton = document.querySelector<HTMLButtonElement>("#prove")!;
const vproveButton = document.querySelector<HTMLButtonElement>("#vprove")!;
const proveWebButton = document.querySelector<HTMLButtonElement>("#proveweb")!;
setupRequestProveButton(twitterProofButton);
setupVProverButton(vproveButton);
setupProveWebButton(proveWebButton);
setupVerifyButton(document.querySelector<HTMLButtonElement>("#vverify")!);
