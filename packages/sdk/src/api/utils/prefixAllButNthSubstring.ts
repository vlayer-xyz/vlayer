export function prefixAllButNthSubstring(
  str: string,
  substr: string,
  n: number,
) {
  let occurrence = 0;
  return str.replace(new RegExp(substr, "gi"), (match) => {
    return occurrence++ === n ? match : `x-${match}`;
  });
}
