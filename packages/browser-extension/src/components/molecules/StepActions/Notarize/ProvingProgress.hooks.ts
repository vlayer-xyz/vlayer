import { useEffect, useState } from "react";
import { useInterval } from "usehooks-ts";
import { ProvingStatus } from "./types";
import { match } from "ts-pattern";

export const useProvingProgress = (props: {
  isVisible: boolean;
  provingStatus: ProvingStatus;
}) => {
  const [progress, setProgress] = useState(0);
  const [title, setTitle] = useState("");
  const [subtitle, setSubtitle] = useState("");
  const [stepIndex, setStepIndex] = useState<number | null>(null);
  const [dataTestId, setDataTestId] = useState<string | null>(null);
  useInterval(
    () => {
      if (props.provingStatus === ProvingStatus.Done) {
        setProgress(100);
      } else {
        setProgress(progress + 1);
      }
    },
    progress == 100 ? null : 2400,
  );

  useEffect(() => {
    if (props.provingStatus === ProvingStatus.Done) {
      setProgress(100);
    }
  }, [props.provingStatus]);

  useEffect(() => {
    match(props.provingStatus)
      .with(ProvingStatus.Web, () => {
        setTitle("Generating Web Proof");
        setSubtitle("This takes a while. Don’t close your browser.");
        setStepIndex(1);
        setDataTestId("step_proving_web");
      })
      .with(ProvingStatus.Zk, () => {
        setTitle("Generating ZK Proof");
        setSubtitle("This takes a while. Don’t close your browser.");
        setStepIndex(2);
        setDataTestId("step_proving_zk");
      });
  }, [props.provingStatus]);

  return {
    progress,
    title,
    subtitle,
    isVisible: props.isVisible,
    stepIndex: stepIndex,
    stepCount: 2,
    showDescription: props.provingStatus !== ProvingStatus.Done,
    dataTestId: dataTestId,
  };
};
