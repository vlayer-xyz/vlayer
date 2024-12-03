export function prefixAllButNthSubstring(
  str: string,
  pattern: RegExp,
  substringsCount: number,
  skippedIndex: number,
) {
  let occurrence = 0;
  return str.replace(pattern, (match) => {
    return occurrence++ === skippedIndex || occurrence > substringsCount
      ? match
      : `X-${match}`;
  });
}
