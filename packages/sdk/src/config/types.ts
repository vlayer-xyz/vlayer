import { z } from "zod";
import { configSchema } from "./schema";

export type VlayerContextConfig = z.infer<typeof configSchema>;
