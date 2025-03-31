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
  .get("/regular_json", ({ request, set }) => {
    const url = request.url;
    const queryParams = new URLSearchParams(url.split("?")[1]);
    if (queryParams.get("auth") !== "s3cret_t0ken") {
      set.status = 403;
      return {
        success: false,
        error_message: "Missing or wrong authentication",
      };
    }
    if (queryParams.get("are_you_sure") !== "yes") {
      return {
        success: false,
        error_message: "Missing or incorrect query parameter 'are_you_sure'",
      };
    }
    return { success: true, name: "Gandalf", greeting: "Hello, Frodo!" };
  })
  .get("/json_two_bytes_char", () => {
    return { success: true, place: "Barad-dûr" };
  })
  .get("/json_three_bytes_char", () => {
    return { success: true, name: "عبد الله" };
  })
  .put("/update_resource", ({ body }) => {
    const { name } = body as { name: string };
    return { success: true, updatedName: name };
  })
  .get("/auth_header_require", ({ request, set }) => {
    const auth = request.headers.get("Authorization");
    if (auth !== "s3cret_t0ken") {
      set.status = 403;
      return {
        success: false,
        error_message: "Missing or wrong authentication",
      };
    }
    return {
      success: true,
      name: "Tom Bombadil",
      greeting: "Old Tom Bombadil is a merry fellow!",
    };
  })
  .use(cors())
  .listen(3011);
