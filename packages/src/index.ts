export {
  type CallParams,
  type CallContext,
  v_call
} from "./api/v_call";

export {
  getContractSpec,
  prove,
} from "./api/prover";

export * as helpers from "./api/helpers";

// Temp solution to avoid breaking changes on other prove scripts, 
// will change everywhere if we fine with changing helpers => testHelpers
export * as testHelpers from './api/helpers'; 