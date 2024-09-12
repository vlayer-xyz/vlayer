import vlayerLogo from "/vlayer_logo.svg";
import { VlayerTheme } from "@vlayer/components";
import { Button, Grid, TextField } from "@radix-ui/themes";
function App() {
  return (
    <VlayerTheme>
      <Grid columns="1" gapY="4" top="8">
        <div>
          <img src={vlayerLogo} alt="Vlayer Logo" />
        </div>
        <Button variant="soft">Request web proof </Button>

        <TextField.Root placeholder="Provide url... ">
          <TextField.Slot></TextField.Slot>
        </TextField.Root>
      </Grid>
    </VlayerTheme>
  );
}

export default App;
