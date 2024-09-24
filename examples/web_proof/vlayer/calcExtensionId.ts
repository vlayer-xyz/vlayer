import path from "node:path";

const envPath = path.resolve(__dirname, ".env.development");

try {
  let envContent;

  try {
    const envFile = Bun.file(envPath);
    envContent = await envFile.text();
  } catch (err: unknown) {
    if ((err as NodeJS.ErrnoException).code !== "ENOENT") {
      envContent = "";
    }
  }

  if (!envContent) {
    envContent = "";
  }
  const getExtensionId = async () => {
    const { stdout } = await Bun.spawn([
      "bash",
      "../../../bash/generate-extension-id.sh",
    ]);
    return new Response(stdout).text();
  };

  const extensionId = await getExtensionId();

  const regex = /^VITE_EXTENSION_ID=.*/m;

  if (regex.test(envContent)) {
    envContent = envContent.replace(
      regex,
      `VITE_EXTENSION_ID=${extensionId.trim()}`,
    );
  } else {
    envContent += `VITE_EXTENSION_ID=${extensionId}`.trim() + "\n";
  }
  await Bun.write(envPath, envContent);
} catch (err) {
  console.error("Error updating the .env.development file:", err);
}
