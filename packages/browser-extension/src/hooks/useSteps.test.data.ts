import { HistoryItem } from "../state/history.ts";
import { StepStatus } from "constants/step.ts";
import { WebProofStep } from "../web-proof-commons";

export const steps = [
  {
    url: "https://example.com/start",
    label: "Start Page",
    step: "startPage",
  },
  {
    url: "https://example.com/*/expect",
    label: "Expect URL",
    step: "expectUrl",
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
      history: [] as HistoryItem[],
      id: "Empty history",
    },
    output: [StepStatus.Current, StepStatus.Further, StepStatus.Further],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/start", ready: true },
      ] as HistoryItem[],
      id: "Start page visited ",
    },
    output: [StepStatus.Completed, StepStatus.Current, StepStatus.Further],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/expect", ready: true },
      ] as HistoryItem[],
      id: "Expect page visited without visiting start page ",
    },
    output: [StepStatus.Current, StepStatus.Further, StepStatus.Further],
  },
  {
    input: {
      isZkProvingDone: true,
      history: [
        { url: "https://example.com/notarize", ready: true },
      ] as HistoryItem[],
      id: "Notarize page visited and proof is in place but without visiting start page ",
    },
    output: [StepStatus.Current, StepStatus.Further, StepStatus.Further],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/start", ready: true },
        { url: "https://example.com/path/expect", ready: true },
      ] as HistoryItem[],
      id: "Expect page visited and start page visited ",
    },
    output: [StepStatus.Completed, StepStatus.Completed, StepStatus.Further],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/start", ready: true },
        { url: "https://example.com/path/expect", ready: false },
      ] as HistoryItem[],
      id: "Expect page visited ( no cookies) and start page visited ",
    },
    output: [StepStatus.Completed, StepStatus.Current, StepStatus.Further],
  },
  {
    input: {
      isZkProvingDone: false,
      history: [
        { url: "https://example.com/start", ready: true },
        { url: "https://example.com/htap/expect", ready: true },
        { url: "https://example.com/notarize", ready: true },
      ] as HistoryItem[],
      id: "All pages visited but no proof",
    },
    output: [StepStatus.Completed, StepStatus.Completed, StepStatus.Current],
  },
  {
    input: {
      isZkProvingDone: true,
      history: [
        { url: "https://example.com/start", ready: true },
        { url: "https://example.com/htap/expect", ready: true },
        { url: "http://example.com/notarize", ready: true },
      ] as HistoryItem[],
      id: "All data in place",
    },
    output: [StepStatus.Completed, StepStatus.Completed, StepStatus.Completed],
  },
];
