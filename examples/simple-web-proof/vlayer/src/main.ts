import "./style.css";
import { setupProveWebButton, setupVerifyButton } from "./prove";
document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div class="container">
    <div class="logoContainer">
      <img src="vlayer_logo.svg" />
    </div>
    <button id="proveweb" style="margin-top: 10px">Request webproof of web page</button>
    <button id="vverify" style="margin-top: 10px">Call vlayer verifier</button>

  </div>
`;

const proveWebButton = document.querySelector<HTMLButtonElement>("#proveweb")!;
setupProveWebButton(proveWebButton);
setupVerifyButton(document.querySelector<HTMLButtonElement>("#vverify")!);
