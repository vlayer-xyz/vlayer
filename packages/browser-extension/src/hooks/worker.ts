import * as Comlink from 'comlink';
import init, { Prover, Presentation } from 'tlsn-js';

Comlink.expose({
  init,
  Prover,
  Presentation,
});
