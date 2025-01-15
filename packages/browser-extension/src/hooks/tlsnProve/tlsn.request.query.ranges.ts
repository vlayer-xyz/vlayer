const calculateRequestQueryParamsRanges = (
  urlQueryParams: string[],
  url: string,
  offset: number,
) => {
  return urlQueryParams.map((param) => {
    const stepOverFirstAmpersand = 1;
    const start =
      offset + url.indexOf("&" + param + "=") + stepOverFirstAmpersand;

    const secondAmpersandPosition = url.indexOf("&", start);
    const end =
      secondAmpersandPosition !== -1
        ? offset + secondAmpersandPosition
        : offset + url.length;
    return {
      start,
      end,
    };
  });
};

export { calculateRequestQueryParamsRanges };
