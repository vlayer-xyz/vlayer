import React from "react";
import { Button, Theme, Grid, Spinner, Card } from "@radix-ui/themes";
import browser from "webextension-polyfill";
import {
  useTlsnProover,
  useProofContext,
  TlsnProofContextProvider,
} from "../context";

const BackButton = () => {
  const { backUrl } = useProofContext();
  const { proof } = useTlsnProover();
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
  const { prove, proof, isProving, hasDataForProof } = useTlsnProover();

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
  const { hasDataForProof } = useTlsnProover();
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
  const { proof } = useTlsnProover();
  console.log(proof);
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
        <Grid columns="8" gapY="4" top="16" style={{ marginTop: "80px" }}>
          <div style={{ gridColumn: "span 1" }}></div>
          <div style={{ gridColumn: "span 6" }}>
            <Grid columns="1" gapY="4">
              <GoToPageButton />
              <ProofButton />
              <BackButton />
              <Proof />
            </Grid>
          </div>
        </Grid>
      </Theme>
    </TlsnProofContextProvider>
  );
};

export default SidePanel;
