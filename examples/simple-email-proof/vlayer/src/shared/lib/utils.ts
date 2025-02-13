export const getStrFromFile = (file: File): Promise<string> => {
  const reader = new FileReader();

  return new Promise((resolve, reject) => {
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = () => reject(new Error("Reader error"));
    reader.readAsText(file);
  });
};

export const usePrivateKey =
  !import.meta.env.VITE_USE_WINDOW_ETHEREUM_TRANSPORT ||
  Boolean(import.meta.env.VITE_PRIVATE_KEY);
