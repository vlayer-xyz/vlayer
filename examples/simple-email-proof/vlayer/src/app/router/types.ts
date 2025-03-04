export enum STEP_KIND {
  WELCOME,
  CONNECT_WALLET,
  SEND_EMAIL,
  COLLECT_EMAIL,
  MINT_NFT,
  SUCCESS,
}

export type StepMeta = {
  path: string;
  kind: STEP_KIND;
  title: string;
  description: string;
  headerIcon?: string;
  index: number;
  backUrl?: string;
};

export const stepsMeta: Record<STEP_KIND, StepMeta> = {
  [STEP_KIND.WELCOME]: {
    path: "",
    kind: STEP_KIND.WELCOME,
    title: "Domain NFT",
    description:
      "Mint an NFT with your email domain. For example, if you work at Ethereum Foundation, you can mint “ethereum.org” NFT. This showcases Email Proofs.",
    headerIcon: "/img/email-welcome-img.svg",
    index: 0,
  },
  [STEP_KIND.CONNECT_WALLET]: {
    path: "connect-wallet",
    kind: STEP_KIND.CONNECT_WALLET,
    title: "Mail based NFT",
    description:
      "To proceed to the next step, please connect your wallet now by clicking the button below.",
    backUrl: "",
    index: 1,
  },
  [STEP_KIND.SEND_EMAIL]: {
    path: "send-email",
    kind: STEP_KIND.SEND_EMAIL,
    title: "Send Email",
    description:
      "Please copy the details provided below and use them to send the email.",
    backUrl: "connect-wallet",
    index: 2,
  },
  [STEP_KIND.COLLECT_EMAIL]: {
    path: "collect-email",
    kind: STEP_KIND.COLLECT_EMAIL,
    title: "Waiting...",
    description:
      "Our mailbox is processing your email. Please wait a few seconds.",
    backUrl: "send-email",
    index: 2,
  },
  [STEP_KIND.MINT_NFT]: {
    path: "mint-nft",
    kind: STEP_KIND.MINT_NFT,
    title: "Mint NFT",
    description: "Your email is ready for proving and minting.",
    backUrl: "send-email",
    index: 3,
  },
  [STEP_KIND.SUCCESS]: {
    path: "success",
    kind: STEP_KIND.SUCCESS,
    title: "Success",
    description: "",
    headerIcon: "/img/success-icon.svg",
    index: 4,
  },
};
