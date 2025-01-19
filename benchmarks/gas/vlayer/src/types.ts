import { mean, sampleStandardDeviation } from "simple-statistics";

export class MetricsUnpacked {
  gas: Array<number> = [];
  cycles: Array<number> = [];
  times: {
    preflight: Array<number>;
    proving: Array<number>;
  } = {
    preflight: [],
    proving: [],
  };

  toStats(): MetricsStats {
    return {
      gas: new MeanStddev(this.gas),
      cycles: new MeanStddev(this.cycles),
      times: {
        preflight: new MeanStddev(this.times.preflight),
        proving: new MeanStddev(this.times.proving),
      },
    };
  }
}

export class MeanStddev {
  mean: number;
  stddev: number;

  constructor(values: number[]) {
    this.mean = mean(values);
    this.stddev = sampleStandardDeviation(values);
  }
}

export type MetricsStats = {
  gas: MeanStddev;
  cycles: MeanStddev;
  times: {
    preflight: MeanStddev;
    proving: MeanStddev;
  };
};
