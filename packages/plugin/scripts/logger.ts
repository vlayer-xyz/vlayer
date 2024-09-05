import chalk from "chalk";

const log = (message: string, color = chalk.white) => {
  const stars = "*".repeat(20);
  console.log(chalk.bold(stars));
  console.log(color(message));
  console.log(chalk.bold(stars));
};

export const logger = {
  log: log,
  error: (message: string) => log(message, chalk.red),
  info: (message: string) => log(message, chalk.blue),
};
