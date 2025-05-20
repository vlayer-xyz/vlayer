import { BrowsingHistoryItem } from "src/state";
import { StepStatus } from "constants/step.ts";
import { WebProofStep } from "../web-proof-commons";
import { StepTestCase } from "hooks/useSteps.test.helpers.ts";

export const steps = [
  {
    url: "https://example.com/start",
    label: "Start Page",
    step: "startPage",
  },
  {
    url: "https://example.com/redirect",
    label: "Redirect Page",
    step: "redirect",
  },
  {
    url: "https://example.com/*/expect",
    label: "Expect URL",
    step: "expectUrl",
  },
  {
    url: "https://example.com/*/expect",
    label: "Expect user action",
    step: "userAction",
    instruction: {
      text: "Click here now",
    },
    assertion: {
      domElement: "button[data-clicked='true']",
      require: { exist: true },
    },
  },
  {
    url: "(https|http)://example.com/notarize",
    label: "Notarize",
    step: "notarize",
  },
] as WebProofStep[];

export const testData = [
  {
    input: {
      isZkProvingDone: false,
      history: [] as BrowsingHistoryItem[],
      id: "Empty history",
    },
    output: [
      StepStatus.Current,
      StepStatus.Further,
      StepStatus.Further,
      StepStatus.Further,
      StepStatus.Further,
    ],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/start", ready: true },
      ] as BrowsingHistoryItem[],
      id: "Start page visited",
    },
    output: [
      StepStatus.Completed,
      StepStatus.Current,
      StepStatus.Further,
      StepStatus.Further,
      StepStatus.Further,
    ],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/start", ready: true },
        { url: "https://example.com/redirect", ready: true },
      ] as BrowsingHistoryItem[],
      id: "Redirect page visited and start page visited",
    },
    output: [
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Current,
      StepStatus.Further,
      StepStatus.Further,
    ],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/path/expect", ready: true },
      ] as BrowsingHistoryItem[],
      id: "Expect page visited without visiting start page",
    },
    output: [
      StepStatus.Current,
      StepStatus.Further,
      StepStatus.Further,
      StepStatus.Further,
      StepStatus.Further,
    ],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/redirect", ready: true },
      ] as BrowsingHistoryItem[],
      id: "Redirect page visited without visiting start page",
    },
    output: [
      StepStatus.Current,
      StepStatus.Further,
      StepStatus.Further,
      StepStatus.Further,
      StepStatus.Further,
    ],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/start", ready: true },
        { url: "https://example.com/path/expect", ready: true },
      ] as BrowsingHistoryItem[],
      id: "Expect page visited without visiting redirect page",
    },
    output: [
      StepStatus.Completed,
      StepStatus.Current,
      StepStatus.Further,
      StepStatus.Further,
      StepStatus.Further,
    ],
  },
  {
    input: {
      isZkProvingDone: true,
      history: [
        { url: "https://example.com/notarize", ready: true },
      ] as BrowsingHistoryItem[],
      id: "Notarize page visited and proof is in place but without visiting start page",
    },
    output: [
      StepStatus.Current,
      StepStatus.Further,
      StepStatus.Further,
      StepStatus.Further,
      StepStatus.Further,
    ],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/start", ready: true },
        { url: "https://example.com/redirect", ready: true },
        { url: "https://example.com/path/expect", ready: true },
        { url: "https://example.com/path/other", ready: true },
      ] as BrowsingHistoryItem[],
      activeTabContext: {
        url: "https://example.com/path/other",
        innerHTML: "<button data-clicked='false'>Click here now</button>",
      },
      id: "Expect page visited, redirect page visited and start page visited, user action url is not active",
    },
    output: [
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Current,
      StepStatus.Further,
    ],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/start", ready: true },
        { url: "https://example.com/redirect", ready: true },
        { url: "https://example.com/path/expect", ready: true },
      ] as BrowsingHistoryItem[],
      activeTabContext: {
        url: "https://example.com/path/expect",
        innerHTML: "<button data-clicked='false'>Click here now</button>",
      },
      id: "Expect page visited, redirect page visited and start page visited, user action not completed",
    },
    output: [
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Current,
      StepStatus.Further,
    ],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/start", ready: true },
        { url: "https://example.com/redirect", ready: true },
        { url: "https://example.com/path/expect", ready: false },
      ] as BrowsingHistoryItem[],
      activeTabContext: {
        url: "https://example.com/path/expect",
        innerHTML: "<button data-clicked='true'>Click here now</button>",
      },
      id: "Expect page visited ( no cookies), redirect page visited and start page visited, user action completed",
    },
    output: [
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Further,
    ],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/start", ready: true },
        { url: "https://example.com/redirect", ready: true },
        { url: "https://example.com/htap/expect", ready: true },
        { url: "https://example.com/notarize", ready: true },
      ] as BrowsingHistoryItem[],
      activeTabContext: {
        url: "https://example.com/path/expect",
        innerHTML: "<button data-clicked='true'>Click here now</button>",
      },
      id: "All pages visited but no proof",
    },
    output: [
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Current,
    ],
  },
  {
    input: {
      isZkProvingDone: true,
      history: [
        { url: "https://example.com/start", ready: true },
        { url: "https://example.com/redirect", ready: true },
        { url: "https://example.com/htap/expect", ready: true },
        { url: "http://example.com/notarize", ready: true },
      ] as BrowsingHistoryItem[],
      activeTabContext: {
        url: "https://example.com/path/expect",
        innerHTML: "<button data-clicked='true'>Click here now</button>",
      },
      id: "All data in place",
    },
    output: [
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Completed,
      StepStatus.Completed,
    ],
  },
] as const satisfies StepTestCase[];
