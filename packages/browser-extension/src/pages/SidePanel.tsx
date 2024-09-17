import React from "react";
import { Button, Theme, Grid, Spinner} from "@radix-ui/themes";
import browser from "webextension-polyfill";
import { 
	useTlsnProover,
	useProofContext
} from '../hooks'


const BackButton = ({isVisible } : { isVisible : boolean}) => {
	const { backUrl } = useProofContext();
	return isVisible ? <Button onClick={() => {
		browser.tabs.create({ url: backUrl });
	}}>Back</Button> : null;
}

const ProofButton = ( {isVisible} : {isVisible : boolean } ) => {
	const { prove, isProoving, hasDataForProof } = useTlsnProover();
	return isVisible ? <Button disabled={hasDataForProof ? false : true } onClick={()=> {
		prove()
	}}> {
		isProoving ? <Spinner /> : 'Make Proof'
	}  </Button> : null;
}

const GoToPageButton = ({ isVisible} : {isVisible : boolean}) => {
	const { redirectUrl } = useProofContext();
	return isVisible ? <Button variant="soft" onClick={ () => {
		browser.tabs.create({ url: redirectUrl });
	}}> Go to page {redirectUrl} </Button> : null;
}

export default function SidePanel() {
  return (
    <Theme accentColor="violet">
      <Grid columns="8" gapY="4" top="16" style={{ marginTop: "80px" }}>
        <div style={{ gridColumn: "span 1" }}></div>
        <div style={{ gridColumn: "span 6" }}>
          <Grid columns="1" gapY="4">
            <Button variant="soft"> Go to page .. </Button>
            <Button> Make a proof </Button>
          </Grid>
        </div>
      </Grid>
    </Theme>
  );
}
