import { type Hex, type Abi, type ContractFunctionName } from "viem";
import type { ContractFunctionArgsWithout } from "./viem";
import {
  type Branded,
  type ExtensionMessageType,
  type ExtensionMessage,
  type PresentationJSON,
  type WebProofStep,
  type ZkProvingStatus,
} from "../../../web-proof-commons";

export type WebProofRequestInput = {
  logoUrl: string;
  steps: WebProofStep[];
};

export type WebProofRequest = Branded<
  WebProofRequestInput & {
    isWebProof: true;
  },
  "webProof"
>;

export type ProverCallCommitment<
  T extends Abi,
  F extends ContractFunctionName<T>,
> = {
  address: Hex;
  proverAbi: T;
  functionName: F;
  commitmentArgs: ContractFunctionArgsWithout<T, F, { name: "webProof" }>;
  chainId: number;
};

export type GetWebProofArgs<
  T extends Abi,
  F extends ContractFunctionName<T>,
> = {
  proverCallCommitment: ProverCallCommitment<T, F>;
} & WebProofRequestInput;

export type WebProofProvider = {
  getWebProof: <T extends Abi, F extends ContractFunctionName<T>>(
    args: GetWebProofArgs<T, F>,
  ) => Promise<{
    presentationJSON: PresentationJSON;
    decodedTranscript: {
      sent: string;
      recv: string;
    };
  }>;

  requestWebProof: <T extends Abi, F extends ContractFunctionName<T>>(
    args: GetWebProofArgs<T, F>,
  ) => void;

  notifyZkProvingStatus: (status: ZkProvingStatus) => void;

  addEventListeners: <T extends ExtensionMessageType>(
    messageType: T,
    listener: (args: Extract<ExtensionMessage, { type: T }>) => void,
  ) => void;
};

export type WebProofProviderSetup = {
  notaryUrl?: string;
  wsProxyUrl?: string;
};

# Browser Extension Corner Cases Handling

## Corner cases/bugs/tech debt discovered by category

### 1. Critical User Experience Issues
1. [extension] "prf"/"mpc" error from TLSN → performance issue, but add user-friendly handling
2. [extension] TLSN issues/errors - identify categories and define what kind of user-friendly errors we want to display to the user
3. [SDK] provide functionality to detect extension in SDK
4. [SDK] handle extension not installed in SDK
5. [extension] clear last proving state from extension
6. [extension, SDK] 403 (and other non-200) status codes don't return ProofError to webapp → comprehensive handling of non-200 status handling + proper routing of all errors to webapp
7. [SDK] map v_call errors to SDK errors
8. [SDK] add validation off all config/input with meaningful error messages
9. [SDK] "Open extension" instead of "Proving in progress…" when ZK proving
10. [extension] Rare scenario but happening: "Log in" step is ticked, but no "Prove" button

### 2. Improve Testing and Internal DX
1. [extension, SDK, example] [run-web-example.sh](http://run-web-services.sh) script breaks often (it's not tested anywhere in CI) → delete or use in tests
2. [extension] response is not logged in extension on non-200 status code → add overall support for easier debugging and improved logging
3. [extension] Race condition in history updates 
    1. this reads **async** data storage and then update **asynchronusly** but there is no lock. ([gh link](https://github.com/vlayer-xyz/vlayer/blob/main/packages/browser-extension/src/state/history.ts#L39))
4. [extension] Too many history updates in one request cycle 
    1. There are many stages of request we are interested on, we get some data there and immediatelly update history. This gives us miliseconds but we pay the price of generating lot of calls that are hard to dedug. We should handle whole request, esplacially taking into account data are usable only after whole request completion, collect request data and then sync it with storage.  ([GH link](https://github.com/vlayer-xyz/vlayer/blob/main/packages/browser-extension/src/hooks/useTrackHistory.ts))
5. [extension] use hooks in playwirght e2e tests of extension
6. [extension, SDK] Remove old promised based communication
7. [extension] Move components of test-web-app to dedicated files to increase readability
8. [extension] Missing proof validation in e2e
9. [extension] Move test flows to dedicated files, apply DRY.
10. [extension] As part of 15 use POM https://playwright.dev/docs/pom.
11. [extension] Missing visual regression tests.
12. [extension] Handle multiple error cases properly (avoid repetition of common steps)
13. [extension] decide what playwright e2e tests are, consider deleting email tests from test-web-app, as they are already tested in simple-email-proof
14. [extension] consider integration testing `tlsnProve` and errors there

### 3. Documentation
1. [extension] Very old Contributing→Extension part in book

TODO: discuss impact of issues on customers

## Roadmap/plan to address issues

1. Critical User experience issues 
2. Improve testing and internal DX 
3. Documentation