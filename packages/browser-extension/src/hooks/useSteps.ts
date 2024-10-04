// // this is placeholder impolementation
//
// import { StepStatus } from "constants/step";
//
// export const useSteps = (): {
//   status: StepStatus;
//   label: string;
//   kind: "expectUrl" | "notarize" | "startPage";
//   link?: string;
//   buttonText?: string;
// }[] => {
//   return [
//     {
//       status: StepStatus.Completed,
//       label: "Go to app.revolut.com and login",
//       kind: "startPage",
//       link: "https://app.revolut.com/",
//       buttonText: "Go to revolut",
//     },
//     {
//       status: StepStatus.Current,
//       label: "Go to app.revolut.com and login",
//       kind: "startPage",
//       link: "https://app.revolut.com/",
//       buttonText: "Go to revolut",
//     },
//     {
//       status: StepStatus.Further,
//       label: "thirdStep",
//       kind: "expectUrl",
//     },
//     {
//       status: StepStatus.Further,
//       label: "fourthStep",
//       kind: "notarize",
//     },
//   ];
// };
