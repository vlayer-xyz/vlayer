import Menu from "../components/Menu";
import { requestProve, callProver, callVerifier } from "../utils/webProof";

export default function Actions() {
  return (
    <Menu
      requestProve={requestProve}
      callProver={callProver}
      callVerifier={callVerifier}
    />
  );
}
