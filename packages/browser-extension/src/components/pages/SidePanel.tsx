import React from "react";
import { Button, Theme, Grid, Spinner, Card } from "@radix-ui/themes";
import browser from "webextension-polyfill";
import {
  useTlsnProver,
  useProofContext,
  TlsnProofContextProvider,
} from "../../context";

// import { Steps } from "components/organisms/Steps";

const createTab = async (url: string) => {
  await browser.tabs.create({ url });
};

const BackButton = () => {
  const { backUrl } = useProofContext();
  const { proof } = useTlsnProver();
  const handleClick = () => {
    createTab(backUrl).catch((error) => {
      console.error("Error during creating new tab:", error);
    });
  };
  return proof ? <Button onClick={handleClick}>Back</Button> : null;
};

const ProofButton = () => {
  const { prove, proof, isProving, hasDataForProof } = useTlsnProver();
  const handleClick = () => {
    prove().catch((error) => {
      console.error("Error during prove:", error);
    });
  };

  return !proof ? (
    <Button disabled={hasDataForProof ? false : true} onClick={handleClick}>
      {" "}
      {isProving ? <Spinner /> : "Make Proof"}{" "}
    </Button>
  ) : null;
};

const GoToPageButton = () => {
  const { hasDataForProof } = { hasDataForProof: false }; //useTlsnProver();
  const { redirectUrl } = useProofContext();
  const handleClick = () => {
    createTab(redirectUrl).catch((error) => {
      console.error("Error during creating new tab:", error);
    });
  };
  return !hasDataForProof ? (
    <Button variant="soft" onClick={handleClick}>
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
              <Grid columns="1" gapY="4">
                <GoToPageButton />
                <ProofButton />
                <BackButton />
                <Proof />
              </Grid>
              {/*<Steps />*/}
            </div>
          </Grid>
        </div>
      </Theme>
    </TlsnProofContextProvider>
  );
};

export default SidePanel;
