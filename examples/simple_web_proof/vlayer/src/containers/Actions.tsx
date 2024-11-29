import Menu from "../components/Menu";
import { requestProve } from "../utils/webProof";

export default function Actions() {
  return <Menu requestProve={requestProve} />;
}
