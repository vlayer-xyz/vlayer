const calculateRequestQueriesRanges = (url_query: string[], raw: string) => {
  return url_query.map((query) => {
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
