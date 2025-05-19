export { preverifyEmail } from "./api/email/preverify";
export { createVlayerClient } from "./api/lib/client";
export { createExtensionWebProofProvider } from "./api/webProof/providers/extension";

export * from "./api/lib/types";

export * from "./web-proof-commons/utils";
export * from "./web-proof-commons/types/message";
export {
  type RedactionItem,
  type RedactionConfig,
  RedactionItemsArray as RedactionConfigSchema,
} from "./web-proof-commons/types/redaction";
export * from "./web-proof-commons/error";

export * from "./utils";
