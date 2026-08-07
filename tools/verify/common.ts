import { spawnSync, type SpawnSyncReturns } from "node:child_process";
import { lstatSync, mkdtempSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, join, relative } from "node:path";
import { PROJECT_ROOT, findExecutable } from "../native/project.js";

type Check = {
  detail: string;
  name: string;
  ok: boolean;
  skipped?: boolean;
};

function run(
  command: string[],
  cwd: string = PROJECT_ROOT,
): SpawnSyncReturns<string> {
  const executable = command[0];
  if (!executable) throw new Error("Cannot run an empty command");
  return spawnSync(executable, command.slice(1), {
      cwd,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
  });
}

function commandDetail(result: SpawnSyncReturns<string>): string {
  if (result.error) return result.error.message;
  return (result.stderr || result.stdout).trim() || "passed";
}

function relativePath(path: string): string {
  return relative(PROJECT_ROOT, path);
}

function createTemporaryDirectory(prefix: string): string {
  return mkdtempSync(join(tmpdir(), prefix));
}

function removeDirectory(path: string): void {
  rmSync(path, { force: true, recursive: true });
}

function walkFiles(root: string, ignoredParts: ReadonlySet<string> = new Set()): string[] {
  const entries = readdirSync(root).map((name) => join(root, name));
  return entries.flatMap((path) => {
      if (ignoredParts.has(basename(path))) return [];
      const status = lstatSync(path);
      if (status.isSymbolicLink()) return [];
      if (status.isDirectory()) return walkFiles(path, ignoredParts);
      return [path];
  });
}

export {
  PROJECT_ROOT,
  type Check,
  commandDetail,
  createTemporaryDirectory,
  findExecutable,
  relativePath,
  removeDirectory,
  run,
  walkFiles,
};
