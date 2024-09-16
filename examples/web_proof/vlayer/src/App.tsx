import vlayerLogo from "/vlayer_logo.svg";
import { VlayerTheme } from "@vlayer/components";
import { Button, Grid } from "@radix-ui/themes";
function App() {
  return (
    <VlayerTheme>
      <Grid columns="1" gapY="4" top="8">
        <div>
          <img src={vlayerLogo} alt="Vlayer Logo" />
        </div>
        <Button
          variant="soft"
          onClick={async () => {
            console.log("Requesting twitter web proof");
            chrome.runtime.sendMessage(
              import.meta.env.VITE_EXTENSION_ID,
              {
                type: "REQUEST_WEB_PROOF",
              },
              (response) => {
                console.log("Response from extension", response);
              },
            );
          }}
        >
          Request X web proof{" "}
        </Button>
      </Grid>
    </VlayerTheme>
  );
}

export default App;
