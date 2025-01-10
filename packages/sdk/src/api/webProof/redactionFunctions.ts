import {
  type RedactRequestHeaders,
  type RedactRequestHeadersExcept,
  type RedactRequestUrlQuery,
  type RedactRequestUrlQueryExcept,
  type RedactResponseHeaders,
  type RedactResponseHeadersExcept,
  type RedactResponseJsonBody,
  type RedactResponseJsonBodyExcept,
} from "../../web-proof-commons/types/message";

export const request = {
  headers: {
    redact: (headers: string[]) => {
      return {
        request: {
          headers: headers,
        },
      } as RedactRequestHeaders;
    },
    redactAllExcept: (headers: string[]) => {
      return {
        request: {
          headers_except: headers,
        },
      } as RedactRequestHeadersExcept;
    },
  },
  url: {
    redactQueryParams: (params: string[]) => {
      return {
        request: {
          url_query: params,
        },
      } as RedactRequestUrlQuery;
    },
    redactAllQueryParamsExcept: (params: string[]) => {
      return {
        request: {
          url_query_except: params,
        },
      } as RedactRequestUrlQueryExcept;
    },
  },
};

export const response = {
  headers: {
    redact: (headers: string[]) => {
      return {
        response: {
          headers: headers,
        },
      } as RedactResponseHeaders;
    },
    redactAllExcept: (headers: string[]) => {
      return {
        response: {
          headers_except: headers,
        },
      } as RedactResponseHeadersExcept;
    },
  },
  jsonBody: {
    redact: (fields: string[]) => {
      return {
        response: {
          json_body: fields,
        },
      } as RedactResponseJsonBody;
    },
    redactAllExcept: (fields: string[]) => {
      return {
        response: {
          json_body_except: fields,
        },
      } as RedactResponseJsonBodyExcept;
    },
  },
};
