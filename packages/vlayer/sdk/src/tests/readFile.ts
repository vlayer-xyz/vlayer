import fs from "fs";

export const readFile = (path: string) => fs.readFileSync(path).toString();
