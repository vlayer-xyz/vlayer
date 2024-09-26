export {  v_call } from "./api/v_call";
export type { CallParams, CallContext} from "types/vlayer";
export type { ContractSpec } from "types/ethereum";

export { getContractSpec, prove } from "./api/prover";
export * as testHelpers from "./api/helpers";
export { client as createTestClient } from "./api/helpers";
export { enrichEmail } from "./api/email/enrich.ts";
