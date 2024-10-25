import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const getStrFromFile = (file: File): Promise<string> =>
  new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (event) => {
      if (event.target) {
        resolve(event.target.result as string);
      } else {
        reject(new Error("Event target is null"));
      }
    };
    reader.onerror = (error) => {
      reject(error);
    };
    reader.readAsText(file);
  });
