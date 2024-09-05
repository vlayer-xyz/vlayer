import chalk from "chalk";

const log = (message, color = chalk.white) => {
  const stars = "*".repeat(20);
  console.log(chalk.bold(stars));
  console.log(color(message));
  console.log(chalk.bold(stars));
};

export const logger = {
  log: log,
  error: (message) => log(message, chalk.red),
  info: (message) => log(message, chalk.blue),
};
