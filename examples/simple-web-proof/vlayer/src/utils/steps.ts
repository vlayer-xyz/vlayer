import {
  ConnectWalletStep,
  MintStep,
  ProveStep,
  SuccessStep,
  WelcomeScreen,
} from "../components";

export type Step = {
  kind: STEP_KIND;
  path: string;
  backUrl?: string;
  component: React.ComponentType;
  title: string;
  description: string;
  headerIcon?: string;
};

export enum STEP_KIND {
  WELCOME,
  CONNECT_WALLET,
  START_PROVING,
  MINT,
  SUCCESS,
}
export const steps: Step[] = [
  {
    path: "",
    kind: STEP_KIND.WELCOME,
    component: WelcomeScreen,
    title: "X NFT",
    description:
      "Mint an NFT with your X account. Only owner of account can mint NFT for specific handle. This example demonstrates use of Web Proofs.",
    headerIcon: "/nft-illustration.svg",
  },
  {
    path: "connect-wallet",
    kind: STEP_KIND.CONNECT_WALLET,
    backUrl: "",
    component: ConnectWalletStep,
    title: "X NFT",
    description:
      "To proceed to the next step, please connect your wallet now by clicking the button below.",
  },
  {
    path: "start-proving",
    kind: STEP_KIND.START_PROVING,
    backUrl: "/connect-wallet",
    component: ProveStep,
    title: "X NFT",
    description:
      "Open vlayer browser extension and follow instructions in order to produce the Proof of X account ownership. \n",
  },
  {
    path: "mint",
    kind: STEP_KIND.MINT,
    backUrl: "/start-proving",
    component: MintStep,
    title: "X NFT",
    description: `You are all set to mint your unique X NFT, a true reflection of your verified identity.`,
  },
  {
    path: "success",
    kind: STEP_KIND.SUCCESS,
    component: SuccessStep,
    title: "Success",
    description:
      "You have successfully minted your X NFT. Check the details below.",
    headerIcon: "/success-illustration.svg",
  },
];
