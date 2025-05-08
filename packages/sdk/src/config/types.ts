import { z } from "zod";
import { configSchema } from "./schema";

export type VlayerContextConfig = z.infer<typeof configSchema>;
export type Overrides = { [key: string]: string | number | undefined };
export type DefinedOverrides = { [key: string]: string | number };
