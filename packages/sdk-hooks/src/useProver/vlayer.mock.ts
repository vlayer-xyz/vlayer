import createFetchMock from "vitest-fetch-mock";
import { vi } from "vitest";

type ProverStateSequence = ["pending", ..."pending"[], "error" | "ready"];

export const mockVlayer = (behaviour: ProverStateSequence) => {
  const fetchMocker = createFetchMock(vi);
  const hashStr = "0x1234567890";
  fetchMocker.mockResponseOnce(() => {
    return {
      body: JSON.stringify({
        result: hashStr,
      }),
    };
  });

  behaviour.forEach((state) => {
    fetchMocker.mockResponseOnce(() => {
      return {
        body: JSON.stringify({
          result: state,
        }),
      };
    });
  });
};
