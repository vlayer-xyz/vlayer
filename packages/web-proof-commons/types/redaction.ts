import { z } from "zod";
import type { WebProofStepNotarize, WebProverSessionConfig } from "./message";

export const RedactRequestHeadersSchema = z.object({
  request: z.object({
    headers: z.array(z.string()),
  }),
});

export const RedactRequestHeadersExceptSchema = z.object({
  request: z.object({
    headers_except: z.array(z.string()),
  }),
});

export const RedactRequestUrlQueryParamSchema = z.object({
  request: z.object({
    url_query: z.array(z.string()),
  }),
});

export const RedactRequestUrlQueryParamExceptSchema = z.object({
  request: z.object({
    url_query_except: z.array(z.string()),
  }),
});

export const RedactResponseHeadersSchema = z.object({
  response: z.object({
    headers: z.array(z.string()),
  }),
});

export const RedactResponseHeadersExceptSchema = z.object({
  response: z.object({
    headers_except: z.array(z.string()),
  }),
});

export const RedactResponseJsonBodySchema = z.object({
  response: z.object({
    json_body: z.array(z.string()),
  }),
});

export const RedactResponseJsonBodyExceptSchema = z.object({
  response: z.object({
    json_body_except: z.array(z.string()),
  }),
});

export const RedactionItemSchema = z.union([
  RedactRequestHeadersSchema,
  RedactRequestHeadersExceptSchema,
  RedactRequestUrlQueryParamSchema,
  RedactRequestUrlQueryParamExceptSchema,
  RedactResponseHeadersSchema,
  RedactResponseHeadersExceptSchema,
  RedactResponseJsonBodySchema,
  RedactResponseJsonBodyExceptSchema,
]);

export type RedactRequestHeadersExcept = z.infer<
  typeof RedactRequestHeadersExceptSchema
>;
export type RedactRequestHeaders = z.infer<typeof RedactRequestHeadersSchema>;

export type RedactRequestUrlQueryParam = z.infer<
  typeof RedactRequestUrlQueryParamSchema
>;
export type RedactRequestUrlQueryParamExcept = z.infer<
  typeof RedactRequestUrlQueryParamExceptSchema
>;
export type RedactResponseHeaders = z.infer<typeof RedactResponseHeadersSchema>;

export type RedactResponseJsonBody = z.infer<
  typeof RedactResponseJsonBodySchema
>;
export type RedactResponseJsonBodyExcept = z.infer<
  typeof RedactResponseJsonBodyExceptSchema
>;
export type RedactResponseHeadersExcept = z.infer<
  typeof RedactResponseHeadersExceptSchema
>;

export type RedactionItem = z.infer<typeof RedactionItemSchema>;
// Define the individual types

const checkConflictingItems =
  (items: RedactionItem[]) =>
  (
    getFirstItem: (item: RedactionItem) => boolean,
    getSecondItem: (item: RedactionItem) => boolean,
  ) => {
    const hasFirst = items.some(getFirstItem);
    const hasSecond = items.some(getSecondItem);
    return !(hasFirst && hasSecond);
  };

const ensureNoResponseHeadersConflict = (items: RedactionItem[]) => {
  const hasResponseHeaders = (item: RedactionItem) =>
    "response" in item &&
    "headers" in item.response &&
    item.response.headers.length > 0;

  const hasResponseHeadersExcept = (item: RedactionItem) =>
    "response" in item &&
    "headers_except" in item.response &&
    item.response.headers_except.length > 0;

  return checkConflictingItems(items)(
    hasResponseHeaders,
    hasResponseHeadersExcept,
  );
};

const ensureNoResponseJsonBodyConflict = (items: RedactionItem[]) => {
  const hasResponseJsonBody = (item: RedactionItem) =>
    "response" in item &&
    "json_body" in item.response &&
    item.response.json_body.length > 0;

  const hasResponseJsonBodyExcept = (item: RedactionItem) =>
    "response" in item &&
    "json_body_except" in item.response &&
    item.response.json_body_except.length > 0;

  return checkConflictingItems(items)(
    hasResponseJsonBody,
    hasResponseJsonBodyExcept,
  );
};

const ensureNoRequestHeadersConflict = (items: RedactionItem[]) => {
  const hasRequestHeaders = (item: RedactionItem) =>
    "request" in item &&
    "headers" in item.request &&
    item.request.headers.length > 0;

  const hasRequestHeadersExcept = (item: RedactionItem) =>
    "request" in item &&
    "headers_except" in item.request &&
    item.request.headers_except.length > 0;

  return checkConflictingItems(items)(
    hasRequestHeaders,
    hasRequestHeadersExcept,
  );
};

const ensureNoRequestUrlQueryParamConflict = (items: RedactionItem[]) => {
  const hasRequestUrlQuery = (item: RedactionItem) =>
    "request" in item &&
    "url_query" in item.request &&
    item.request.url_query.length > 0;

  const hasRequestUrlQueryExcept = (item: RedactionItem) =>
    "request" in item &&
    "url_query_except" in item.request &&
    item.request.url_query_except.length > 0;

  return checkConflictingItems(items)(
    hasRequestUrlQuery,
    hasRequestUrlQueryExcept,
  );
};

export const RedactionItemsArray = z
  .array(RedactionItemSchema)
  .refine(ensureNoResponseHeadersConflict, {
    message: "Cannot have both response headers and response headers_except",
  })
  .refine(ensureNoResponseJsonBodyConflict, {
    message:
      "Cannot have both response json_body and response json_body_except",
  })
  .refine(ensureNoRequestHeadersConflict, {
    message: "Cannot have both request headers and request headers_except",
  })
  .refine(ensureNoRequestUrlQueryParamConflict, {
    message: "Cannot have both request url_query and request url_query_except",
  });

export type RedactionConfig = RedactionItem[];

export function getRedactionConfig(
  provingSessionConfig: WebProverSessionConfig,
): RedactionConfig {
  const notarizeStep = provingSessionConfig.steps.find(
    (step): step is WebProofStepNotarize => step.step === "notarize",
  );
  const redactionConfig = notarizeStep !== undefined ? notarizeStep.redact : [];
  return redactionConfig;
}
