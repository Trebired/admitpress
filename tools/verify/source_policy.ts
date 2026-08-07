import { readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { PROJECT_ROOT, type Check, walkFiles } from "./common.js";

type BannedNamePolicy = {
  allowedPaths: Set<string>;
  value: string;
};

function readPackagePolicyNames(): BannedNamePolicy[] {
  const packageData = JSON.parse(
    readFileSync(join(PROJECT_ROOT, "package.json"), "utf8"),
  );
  const config = packageData.config;
  if (!isObject(config)) throw new Error("package.json#config must be set");
  return [
    {
      allowedPaths: documentationPaths(),
      value: packageConfigValue(config, "organization"),
    },
    {
      allowedPaths: applicationNameAllowedPaths(),
      value: packageConfigValue(config, "app"),
    },
  ];
}

function applicationNameAllowedPaths(): Set<string> {
  return new Set([
      ...documentationPaths(),
      "Cargo.toml",
      "crates/ticket-kde/Cargo.toml",
      "packaging/fedora/app.ticket.native.desktop",
  ]);
}

function documentationPaths(): Set<string> {
  return new Set(["CHANGELOG.md", "CONTRIBUTING.md", "README.md"]);
}

function packageConfigValue(
  config: Record<string, unknown>,
  section: string,
): string {
  const sectionData = config[section];
  if (!isObject(sectionData)) {
    throw new Error(`package.json#config.${section} must be set`);
  }
  const value = sectionData.name;
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new Error(`package.json#config.${section}.name must be set`);
  }
  return value.trim().toLowerCase();
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function checkSourcePolicy(): Check {
  const failures: string[] = [];
  const unfinished = new RegExp(
    "\\b(?:" +
      ["to" + "do!", "un" + "implemented!", "TO" + "DO", "FIX" + "ME"].join(
      "|",
    ) +
      ")\\b",
  );
  const badTimeName = new RegExp("\\b(?:time" + "stamp|t" + "s)\\b");
  const bannedNames = readPackagePolicyNames();
  for (const path of policyFiles()) {
    checkPolicyFile(path, failures, unfinished, badTimeName, bannedNames);
  }
  return {
    detail: failures.length === 0 ? "no forbidden patterns" : failures.join("; "),
    name: "Source policy",
    ok: failures.length === 0,
  };
}

function policyFiles(): string[] {
  const ignoredParts = new Set([
      "target",
      ".git",
      "node_modules",
      "dist",
      "data",
  ]);
  const suffixes = new Set([
      ".rs",
      ".qml",
      ".sql",
      ".md",
      ".toml",
      ".sh",
      ".desktop",
      ".yml",
      ".ts",
  ]);
  return walkFiles(PROJECT_ROOT, ignoredParts)
  .filter((path) => hasPolicySuffix(path, suffixes))
  .filter((path) => !path.endsWith("VERIFICATION.md"))
  .filter((path) => !path.split("/").includes("code-discipline"));
}

function hasPolicySuffix(path: string, suffixes: Set<string>): boolean {
  return [...suffixes].some((suffix) => path.endsWith(suffix));
}

function checkPolicyFile(
  path: string,
  failures: string[],
  unfinished: RegExp,
  badTimeName: RegExp,
  bannedNames: BannedNamePolicy[],
): void {
  const text = readFileSync(path, "utf8");
  const lowered = text.toLowerCase();
  const policyPath = relative(PROJECT_ROOT, path);
  if (text.includes(String.fromCharCode(0x2014))) {
    failures.push(`${policyPath} contains an em dash`);
  }
  if (unfinished.test(text)) {
    failures.push(`${policyPath} contains unfinished markers`);
  }
  if (
    (path.endsWith(".rs") || path.endsWith(".sql")) &&
      badTimeName.test(text)
  ) {
    failures.push(`${policyPath} contains a forbidden time field name`);
  }
  for (const bannedName of bannedNames) {
    if (
      lowered.includes(bannedName.value) &&
        !bannedName.allowedPaths.has(policyPath)
    ) {
      failures.push(`${policyPath} contains banned name ${bannedName.value}`);
    }
  }
}

export { checkSourcePolicy };
