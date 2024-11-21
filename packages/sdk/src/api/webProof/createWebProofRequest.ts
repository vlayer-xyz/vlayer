import { WebProofRequest, WebProofRequestInput } from "types/webProofProvider";

const NOTARY_PUB_KEY =
  "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

export const createWebProofRequest = ({
  logoUrl,
  steps,
  notaryPubKey = NOTARY_PUB_KEY,
}: WebProofRequestInput) => {
  return {
    logoUrl,
    steps,
    notaryPubKey,
    isWebProof: true,
  } as WebProofRequest;
};
