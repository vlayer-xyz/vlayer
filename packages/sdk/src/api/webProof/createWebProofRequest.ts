import { WebProofRequest, WebProofRequestInput } from "types/webProofProvider";

const NOTARY_PUB_KEY =
  "-----BEGIN PUBLIC KEY-----\nMFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEZT9nJiwhGESLjwQNnZ2MsZ1xwjGzvmhF\nxFi8Vjzanlidbsc1ngM+s1nzlRkZI5UK9BngzmC27BO0qXxPSepIwQ==\n-----END PUBLIC KEY-----\n";

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
