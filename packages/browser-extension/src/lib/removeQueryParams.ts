export function removeQueryParams(url: string) {
  const urlObj = new URL(url);
  return urlObj.origin + urlObj.pathname;
}
