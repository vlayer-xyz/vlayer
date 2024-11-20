export class VersionError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "VersionError";
  }
}

export function parseVCallResponseError({
  message,
}: {
  message: string | undefined;
}): Error {
  if (message?.startsWith("Unsupported CallGuestID")) {
    return new VersionError(`${message}
    vlayer uses the daily release cycle, and SDK version must match the proving server version.
    Please run "vlayer update" to update the SDK to the latest version.`);
  }
  return new Error(`Error response: ${message ?? "unknown error"}`);
}
