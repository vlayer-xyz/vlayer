import "./style.css";
import { setupProveButton } from "./prove";

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div class="container">
    <div class="logoContainer">
      <img src="vlayer_logo.svg" />
    </div>
    <button id="prove">Prove twitter account</button>
  </div>
`;

const twitterProofButton = document.querySelector<HTMLButtonElement>("#prove")!;
setupProveButton(twitterProofButton);
