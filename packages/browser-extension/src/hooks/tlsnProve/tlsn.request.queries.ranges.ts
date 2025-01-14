const calculateRequestQueriesRanges = (urlQueries: string[], raw: string) => {
  return urlQueries.map((query) => {
    const stepOverFirstAmpersand = 1;
    const start = raw.indexOf("&" + query + "=") + stepOverFirstAmpersand;
    const secondAmpersandPosition = raw.indexOf("&", start);
    const end =
      secondAmpersandPosition !== -1
        ? secondAmpersandPosition
        : raw.indexOf(" ", start);
    return {
      start,
      end,
    };
  });
};

export { calculateRequestQueriesRanges };
