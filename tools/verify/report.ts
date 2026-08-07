import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { PROJECT_ROOT, type Check } from "./common.js";
import {
  checkNativeTools,
  checkPackagingMetadata,
  checkRequiredFiles,
  checkShellScripts,
  checkToml,
} from "./files.js";
import { checkQmlBackendContract, checkQmlModuleContract } from "./qml.js";
import { checkSvgAndRasterization } from "./raster.js";
import { checkSourcePolicy } from "./source_policy.js";
import { checkSourceStructure } from "./source.js";
import { checkMigrationAndWorkflow } from "./workflow.js";

type VerifyOptions = {
  reportPath?: string;
};

function collectChecks(): Check[] {
  return [
    checkRequiredFiles(),
    checkToml(),
    checkMigrationAndWorkflow(),
    checkSvgAndRasterization(),
    checkSourceStructure(),
    checkSourcePolicy(),
    checkPackagingMetadata(),
    checkShellScripts(),
    checkQmlBackendContract(),
    checkQmlModuleContract(),
    ...checkNativeTools(),
  ];
}

function report(checks: Check[]): string {
  const lines = ["# Verification report", ""];
  for (const check of checks) {
    lines.push(`- **${checkStatus(check)}**, ${check.name}: ${check.detail}`);
  }
  lines.push(...scopeLines());
  return lines.join("\n");
}

function checkStatus(check: Check): string {
  if (check.skipped) return "SKIP";
  return check.ok ? "PASS" : "FAIL";
}

function scopeLines(): string[] {
  return [
    "",
    "## Scope",
    "",
    [
      "The database migration, transactional counter workflow, SVG parsing,",
      "and available SVG rasterizer were exercised in this environment.",
      "Rust compilation and QML linting are run automatically when their",
      "toolchains are installed; skipped checks identify missing local tools.",
    ].join(" "),
    "",
  ];
}

function runVerification(arguments_: string[] = Bun.argv.slice(2)): number {
  let options: VerifyOptions;
  try {
    options = parseArguments(arguments_);
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    return 2;
  }
  const checks = collectChecks();
  const text = report(checks);
  console.log(text);
  if (options.reportPath !== undefined) {
    writeFileSync(resolve(PROJECT_ROOT, options.reportPath), text, "utf8");
  }
  return checks.every((check) => check.ok) ? 0 : 1;
}

function parseArguments(arguments_: string[]): VerifyOptions {
  const options: VerifyOptions = {};
  for (let index = 0; index < arguments_.length; ++index) {
    const argument = arguments_[index] ?? "";
    if (argument === "--report") {
      const next = arguments_[index + 1];
      if (next === undefined) throw new Error("--report requires a path");
      options.reportPath = next;
      index += 1;
    } else if (argument.startsWith("--report=")) {
      options.reportPath = argument.slice("--report=".length);
    } else {
      throw new Error(`Unknown verify argument: ${argument}`);
    }
  }
  return options;
}

export { collectChecks, report, runVerification };
