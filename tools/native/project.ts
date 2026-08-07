import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { projectConfigString } from "../project/config.js";

type CommandOptions = {
  env?: NodeJS.ProcessEnv;
};

const PROJECT_ROOT = resolve(import.meta.dir, "../..");
const APP_ID = "app.ticket.native";
const BINARY_NAME = projectConfigString(PROJECT_ROOT, "binaryName");

function commandStatus(
  command: string,
  args: string[],
  options: CommandOptions = {},
): number {
  const result = spawnSync(command, args, {
      cwd: PROJECT_ROOT,
      env: options.env ?? process.env,
      stdio: "inherit",
  });

  if (result.error) {
    console.error(result.error.message);
    return 1;
  }

  return result.status ?? 1;
}

function runOrExit(
  command: string,
  args: string[],
  options: CommandOptions = {},
): void {
  const status = commandStatus(command, args, options);
  if (status !== 0) process.exit(status);
}

function findExecutable(command: string): string | null {
  const result = spawnSync("sh", ["-c", `command -v ${command}`], {
      cwd: PROJECT_ROOT,
      encoding: "utf8",
  });

  if (result.status !== 0) return null;
  const value = result.stdout.trim();
  return value.length > 0 ? value : null;
}

function appendFlag(value: string | undefined, flag: string): string {
  return value && value.length > 0 ? `${value} ${flag}` : flag;
}

function nativeEnvironment(): NodeJS.ProcessEnv {
  const qmake = process.env.QMAKE ?? findExecutable("qmake6") ?? "";
  const environment: NodeJS.ProcessEnv = {
    ...process.env,
    CXXFLAGS: appendFlag(process.env.CXXFLAGS, "-Wno-sfinae-incomplete"),
    QT_QUICK_CONTROLS_STYLE:
    process.env.QT_QUICK_CONTROLS_STYLE ?? "org.kde.desktop",
    TICKET_APP_DATA_DIR:
    process.env.TICKET_APP_DATA_DIR ?? join(PROJECT_ROOT, "data"),
  };

  if (qmake) environment.QMAKE = qmake;
  return environment;
}

function requireCommands(commands: string[]): void {
  const missing = commands.filter((command) => !findExecutable(command));
  if (missing.length === 0) return;

  console.error(`Missing required tool(s): ${missing.join(", ")}`);
  console.error("Install the native dependencies before running this command.");
  process.exit(1);
}

function packageTool(name: string): string {
  return join(PROJECT_ROOT, "node_modules", ".bin", name);
}

function runTypecheck(): void {
  runOrExit(packageTool("tsc"), ["--noEmit"]);
}

function runCargo(args: string[]): void {
  requireCommands(["cargo", "qmake6"]);
  runOrExit("cargo", args, { env: nativeEnvironment() });
}

export {
  APP_ID,
  BINARY_NAME,
  PROJECT_ROOT,
  commandStatus,
  findExecutable,
  nativeEnvironment,
  requireCommands,
  runCargo,
  runOrExit,
  runTypecheck,
};
