import { parse, type SemVer } from "semver";
import { VersionError } from "../lib/errors";

function safeParseSemver(semverString: string): SemVer {
  const parsed = parse(semverString);
  if (parsed === null) {
    throw new VersionError(`Invalid semver string: ${semverString}`);
  }
  return parsed;
}

export function checkVersionCompatibility(
  proverSemver: string,
  sdkSemver: string,
) {
  const { major: proverMajor, minor: proverMinor } =
    safeParseSemver(proverSemver);
  const { major: sdkMajor, minor: sdkMinor } = safeParseSemver(sdkSemver);

  if (proverMajor !== sdkMajor) {
    throw new VersionError(
      `SDK version ${sdkSemver} is incompatible with prover version ${proverSemver}`,
    );
  }

  if (proverMajor === 0 && proverMinor !== sdkMinor) {
    throw new VersionError(
      `SDK version ${sdkSemver} is incompatible with prover version ${proverSemver}`,
    );
  }
}
