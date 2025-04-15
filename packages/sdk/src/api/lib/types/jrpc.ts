import { z } from "zod";

const errorSchema = z.object({
  code: z.number(),
  message: z.string(),
  data: z.unknown().optional(),
});

const idSchema = z.union([z.string(), z.number(), z.null()]);
const tagSchema = z.literal("2.0");

export const responseSchema = z
  .object({
    id: idSchema,
    jsonrpc: tagSchema,
    result: z.unknown().optional(),
    error: errorSchema.optional(),
  })
  .refine(
    (val) =>
      (val.result !== undefined && val.error === undefined) ||
      (val.result === undefined && val.error !== undefined),
    {
      message: "Either 'result' or 'error' must be present, but not both",
      path: ["result", "error"],
    },
  );

export type Response = z.infer<typeof responseSchema>;
