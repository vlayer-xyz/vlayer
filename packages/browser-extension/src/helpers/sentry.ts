import * as Sentry from "@sentry/react";

/**
 * If the sentry DNS url is configured, it will initialize Sentry.
 * We use `version_name` from the extension manifest as a `release` field in the Sentry context.
 */
export function initSentry() {
  if (import.meta.env.VITE_SENTRY_DSN) {
    Sentry.init({
      dsn: import.meta.env.VITE_SENTRY_DSN as string,
      integrations: [],
      release: chrome.runtime.getManifest().version_name,
    });
  }
}
