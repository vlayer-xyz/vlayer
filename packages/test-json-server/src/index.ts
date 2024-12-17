import { Elysia } from "elysia";
import { cors } from "@elysiajs/cors";
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
  .use(cors())
  .listen(3011);
