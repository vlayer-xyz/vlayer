import { z } from "zod";

export const OutputItemSchema = z.object({
  name: z.string(),
  path: z.string(),
});

export type OutputItem = z.infer<typeof OutputItemSchema>;
export type OutputsConfig = OutputItem[];
