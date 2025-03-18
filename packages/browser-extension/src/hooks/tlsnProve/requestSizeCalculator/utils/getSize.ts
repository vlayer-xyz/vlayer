export const getSize = (value: string | undefined) => {
  //important note this only works for utf-8 now, we can read encoding from headers
  //but this will need small refactor as similar mechanism already exists in context of redaction
  return new TextEncoder().encode(value).length;
};
