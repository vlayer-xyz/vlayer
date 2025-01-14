const calculateRequestQueriesRanges = (
  urlQueries: string[],
  url: string,
  offset: number,
) => {
  return urlQueries.map((query) => {
    const stepOverFirstAmpersand = 1;
    const start =
      offset + url.indexOf("&" + query + "=") + stepOverFirstAmpersand;

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

export { calculateRequestQueriesRanges };
