import React from "react";
import { Button, Theme, Grid, Spinner, Card } from "@radix-ui/themes";
import browser from "webextension-polyfill";
import {
  useTlsnProver,
  useProofContext,
  TlsnProofContextProvider,
} from "../context";

import { Steps } from "components/organisms/Steps";

const BackButton = () => {
  const { backUrl } = useProofContext();
  const { proof } = useTlsnProver();
  return proof ? (
    <Button
      onClick={() => {
        browser.tabs.create({ url: backUrl });
      }}
    >
      Back
    </Button>
  ) : null;
};

const ProofButton = () => {
  const { prove, proof, isProving, hasDataForProof } = useTlsnProver();

  return !proof ? (
    <Button
      disabled={hasDataForProof ? false : true}
      onClick={() => {
        prove();
      }}
    >
      {" "}
      {isProving ? <Spinner /> : "Make Proof"}{" "}
    </Button>
  ) : null;
};

const GoToPageButton = () => {
  const { hasDataForProof } = { hasDataForProof: false }; //useTlsnProver();
  const { redirectUrl } = useProofContext();
  return !hasDataForProof ? (
    <Button
      variant="soft"
      onClick={() => {
        browser.tabs.create({ url: redirectUrl });
      }}
    >
      {" "}
      Go to page {redirectUrl}{" "}
    </Button>
  ) : null;
};

const Proof = () => {
  const { proof } = useTlsnProver();
  return proof ? (
    <Card>
      <pre
        style={{
          textWrap: "balance",
          fontSize: "12px",
        }}
      >
        {JSON.stringify(proof, null, 2)}
      </pre>
    </Card>
  ) : null;
};

const SidePanel = () => {
  return (
    <TlsnProofContextProvider>
      <Theme accentColor="violet">
        <div style={{ fontFamily: "Sora" }}>
          <Grid columns="10" gapY="4" top="16" style={{ marginTop: "80px" }}>
            <div style={{ gridColumn: "span 9" }}>
              {/*<Grid columns="1" gapY="4">*/}
              {/*  <GoToPageButton />*/}
              {/*  <ProofButton />*/}
              {/*  <BackButton />*/}
              {/*  <Proof />*/}
              {/*</Grid>*/}
              <Steps />
            </div>
          </Grid>
        </div>
      </Theme>
    </TlsnProofContextProvider>
  );
};

export default SidePanel;
