import { z } from "zod";

export const claimsSchema = z.object({
  host: z.string(),
  port: z.number(),
  sub: z.string(),
  exp: z.number(),
});

export type Claims = z.infer<typeof claimsSchema>;
