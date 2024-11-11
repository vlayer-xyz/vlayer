import { StepStatus } from "constants/step";

export type NotarizeStepActionProps = {
  isVisited: boolean;
  buttonText: string;
  link: string;
  status: StepStatus;
};

export enum ProvingStatus {
  NotStared,
  Web,
  Zk,
  Done,
}
