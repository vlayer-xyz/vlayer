import "./style.css";
import { setupProveWebButton, setupVerifyButton } from "./prove";
document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div class="container">
    <div class="logoContainer">
      <img src="vlayer_logo.svg" />
    </div>
    <button id="prove">Request prove web of twitter account</button>
    <button id="vverify" style="margin-top: 10px">Call vlayer verifier</button>

  </div>
`;

setupProveWebButton(document.querySelector<HTMLButtonElement>("#prove")!);
setupVerifyButton(document.querySelector<HTMLButtonElement>("#vverify")!);
