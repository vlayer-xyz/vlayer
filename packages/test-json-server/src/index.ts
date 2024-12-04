import { Elysia } from "elysia";
import { cors } from "@elysiajs/cors";
// secure the api not to be killed maybe one day
new Elysia({
  serve: {
    tls: {
      key: Bun.file("certs/lotr-api_online.key"),
      cert: Bun.file("certs/lotr-api_online.crt"),
    },
  },
})
  .get("/regular_json", () => {
    return { name: "Gandalf" };
  })
  .get("/json_two_bytes_char", () => {
    return { place: "Barad-dûr" };
  })
  .get("/json_three_bytes_char", () => {
    return { name: "عبد الله" };
  })
  //TODO : move to plugin
  .post(
    "/snapshot",
    ({ url, return_data }: { url: string; return_data: object }) => {
      console.log(url, return_data);
      //TODO this should create a snapshot of the request
      // so we use is and serve to avoid complicated flows
      // leading to the problem of crea ting a snapshot
    },
  )
  .get("/*", () => {
    // TODO check if url match known snapshot url and if so
    // return snapshot data
  })
  .use(cors())
  .listen(3011);
