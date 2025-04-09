import { z } from "zod";

const errorSchema = z.object({
  error: z.string(),
});

type Error = z.infer<typeof errorSchema>;

export { errorSchema, type Error };
