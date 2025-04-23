import { z } from "zod";

export const claimsSchema = z.object({
  host: z.string().optional(),
  port: z.number().optional(),
  sub: z.string(),
  exp: z.number(),
});

export type Claims = z.infer<typeof claimsSchema>;
