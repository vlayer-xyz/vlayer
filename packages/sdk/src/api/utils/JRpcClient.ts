import debug from "debug";

const log = debug("vlayer:JRpcClient");

export class Client {
  url: string = "http://127.0.0.1:3000";
  token: string | undefined = undefined;

  constructor(url: string, token?: string) {
    this.url = url;
    this.token = token;
  }

  public async send(method: string, params: any): Promise<object> {
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
    };
    if (this.token !== undefined) {
      headers["Authorization"] = "Bearer " + this.token;
    }
    const response = await fetch(this.url, {
      method: "POST",
      body: JSON.stringify({
        method,
        params,
        id: 1,
        jsonrpc: "2.0",
      }),
      headers,
    });
    log("response", response);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    const response_json = await response.json();
    log("response_json", response_json);
    this.assertObject(response_json);
    return response_json;
  }

  assertObject(x: unknown): asserts x is object {
    if (typeof x !== "object") {
      throw new Error("Expected object");
    }
  }
}
