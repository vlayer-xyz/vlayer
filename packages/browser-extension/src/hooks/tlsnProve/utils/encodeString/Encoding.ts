import { match } from "ts-pattern";

export enum Encoding {
  UTF8 = "utf-8",
  UTF16 = "utf-16",
}

export const encoder = (encoding: Encoding) => {
  return match(encoding)
    .with(Encoding.UTF8, () => {
      return new TextEncoder();
    })
    .with(Encoding.UTF16, () => {
      return {
        encode: (str: string) => {
          return Uint8Array.from([
            ...(function* utf16Bytes(str: string) {
              for (let i = 0; i < str.length; i++) {
                const charCode = str.charCodeAt(i);
                yield (charCode & 0xff00) >> 8;
                yield charCode & 0x00ff;
              }
            })(str),
          ]);
        },
      };
    })
    .exhaustive();
};
