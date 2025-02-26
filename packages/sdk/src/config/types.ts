import { z } from "zod";
import { configSchema } from "./utils/schema";

export type VlayerContextConfig = z.infer<typeof configSchema>;
