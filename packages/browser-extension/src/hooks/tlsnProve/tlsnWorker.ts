import { expose } from "comlink";
import init, { Prover, Presentation } from "tlsn-js";

expose({
  init,
  Prover,
  Presentation,
});
