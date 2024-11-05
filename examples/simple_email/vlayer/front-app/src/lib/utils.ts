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
      console.error(error);
      reject(new Error("Reader error"));
    };
    reader.readAsText(file);
  });
