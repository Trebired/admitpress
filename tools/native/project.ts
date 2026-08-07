import { spawnSync } from "node:child_process";
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { dirname, join, resolve } from "node:path";
import { homedir } from "node:os";
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

function escapedDesktopExec(value: string): string {
  return value.replace(/\\/g, "\\\\").replace(/ /g, "\\ ");
}

function desktopExecCommand(args: string[]): string {
  return args.map(escapedDesktopExec).join(" ");
}

function dataHome(): string {
  return process.env.XDG_DATA_HOME ?? join(homedir(), ".local", "share");
}

function installDesktopFile(): void {
  const target = join(dataHome(), "applications", `${APP_ID}.desktop`);
  const source = join(PROJECT_ROOT, "packaging", "fedora", `${APP_ID}.desktop`);
  const template = readFileSync(source, "utf8");
  const content = template.replace(
    /^Exec=.*$/m,
    `Exec=${desktopExecCommand(["bun", "--cwd", PROJECT_ROOT, "run", "dev"])}`,
  );
  mkdirSync(dirname(target), { recursive: true });
  writeFileSync(target, content, "utf8");
}

function copyPackagedFile(sourceName: string, targetDirectory: string): void {
  const source = join(PROJECT_ROOT, "packaging", "fedora", sourceName);
  const target = join(targetDirectory, sourceName);
  mkdirSync(dirname(target), { recursive: true });
  copyFileSync(source, target);
}

function refreshDesktopCaches(root: string): void {
  const applications = join(root, "applications");
  const icons = join(root, "icons", "hicolor");
  rmSync(join(icons, "icon-theme.cache"), { force: true });

  const refreshes: [string, string[]][] = [
    ["update-desktop-database", [applications]],
    ["gtk-update-icon-cache", ["--force", "--quiet", icons]],
    ["kbuildsycoca6", ["--noincremental"]],
  ];

  for (const [command, args] of refreshes) {
    if (!findExecutable(command)) continue;
    spawnSync(command, args, { cwd: PROJECT_ROOT, stdio: "ignore" });
  }
}

function systemDesktopEntry(): boolean {
  return ["/usr/local/share", "/usr/share"].some((root) =>
    existsSync(join(root, "applications", `${APP_ID}.desktop`)),
  );
}

function removeUserDesktopIntegration(root: string): void {
  const targets = [
    join(root, "applications", `${APP_ID}.desktop`),
    join(root, "metainfo", `${APP_ID}.metainfo.xml`),
    join(root, "icons", "hicolor", "scalable", "apps", `${APP_ID}.svg`),
  ];
  for (const target of targets) rmSync(target, { force: true });
}

function installDesktopIntegration(): void {
  const root = dataHome();
  if (systemDesktopEntry()) {
    removeUserDesktopIntegration(root);
    refreshDesktopCaches(root);
    return;
  }
  installDesktopFile();
  copyPackagedFile(
    `${APP_ID}.svg`,
    join(root, "icons", "hicolor", "scalable", "apps"),
  );
  copyPackagedFile(`${APP_ID}.metainfo.xml`, join(root, "metainfo"));
  refreshDesktopCaches(root);
}

export {
  APP_ID,
  BINARY_NAME,
  PROJECT_ROOT,
  commandStatus,
  findExecutable,
  installDesktopIntegration,
  nativeEnvironment,
  requireCommands,
  runCargo,
  runOrExit,
  runTypecheck,
};
