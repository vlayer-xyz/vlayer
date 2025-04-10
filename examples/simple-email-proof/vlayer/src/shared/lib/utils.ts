export const getStrFromFile = (file: File): Promise<string> => {
  const reader = new FileReader();

  return new Promise((resolve, reject) => {
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = () => reject(new Error("Reader error"));
    reader.readAsText(file);
  });
};

export const truncateHashOrAddr = (hash: string | null) =>
  hash ? `${hash.slice(0, 4)}...${hash.slice(-4)}` : "";
