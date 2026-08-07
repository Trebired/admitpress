import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import {
  PROJECT_ROOT,
  type Check,
  commandDetail,
  findExecutable,
  relativePath,
  run,
} from "./common.js";
import { projectConfigString } from "../project/config.js";
import { validateXmlText } from "./xml.js";

type CargoPackage = {
  manifest_path?: unknown;
};

type CargoMetadata = {
  packages?: unknown;
};

function checkRequiredFiles(): Check {
  const required = [
    "Cargo.toml",
    "README.md",
    "crates/ticket-core/Cargo.toml",
    "crates/ticket-core/src/service.rs",
    "crates/ticket-core/src/storage.rs",
    "crates/ticket-core/src/renderer.rs",
    "crates/ticket-core/migrations/0001_initial.sql",
    "crates/ticket-kde/Cargo.toml",
    "crates/ticket-kde/build.rs",
    "crates/ticket-kde/src/main.rs",
    "crates/ticket-kde/src/backend.rs",
    "crates/ticket-kde/src/qml/app/main.qml",
    "packaging/fedora/app.ticket.native.desktop",
    "packaging/fedora/app.ticket.native.svg",
    "packaging/fedora/app.ticket.native.metainfo.xml",
    "rust-toolchain.toml",
  ];
  const missing = required.filter(
    (path) => !existsSync(join(PROJECT_ROOT, path)),
  );
  return {
    detail: missing.length === 0 ? "all present" : missing.join(", "),
    name: "Required files",
    ok: missing.length === 0,
  };
}

function checkToml(): Check {
  const cargo = findExecutable("cargo");
  if (cargo === null) {
    return {
      detail: "cargo is not installed",
      name: "TOML parsing",
      ok: false,
    };
  }
  const checked = run([
      cargo,
      "metadata",
      "--no-deps",
      "--format-version",
      "1",
  ]);
  if (checked.status !== 0) {
    return { detail: commandDetail(checked), name: "TOML parsing", ok: false };
  }
  const members = cargoWorkspaceMembers(checked.stdout);
  const expected = new Set(["crates/ticket-core", "crates/ticket-kde"]);
  const ok = [...expected].every((member) => members.includes(member));
  return {
    detail: `parsed ${members.length + 1} files, workspace members: ${displayList(members)}`,
    name: "TOML parsing",
    ok,
  };
}

function cargoWorkspaceMembers(stdout: string): string[] {
  const metadata = JSON.parse(stdout) as CargoMetadata;
  if (!Array.isArray(metadata.packages)) return [];
  return metadata.packages
  .map((item) => packageManifestPath(item))
  .filter((path) => path !== null)
  .map((path) => relative(PROJECT_ROOT, dirname(path)))
  .filter((path) => path.startsWith("crates/"))
  .sort();
}

function packageManifestPath(value: unknown): string | null {
  const item = value as CargoPackage;
  return typeof item.manifest_path === "string" ? item.manifest_path : null;
}

function displayList(values: string[]): string {
  return `[${values.map((value) => `'${value}'`).join(", ")}]`;
}

function checkPackagingMetadata(): Check {
  const files = [
    "packaging/fedora/app.ticket.native.svg",
    "packaging/fedora/app.ticket.native.metainfo.xml",
  ];
  const failures = files.flatMap(validateXmlFile);
  const desktop = readFileSync(
    join(PROJECT_ROOT, "packaging/fedora/app.ticket.native.desktop"),
    "utf8",
  );
  const required = [
    "Type=Application",
    `Exec=${projectConfigString(PROJECT_ROOT, "binaryName")}`,
    "Icon=app.ticket.native",
    "Terminal=false",
  ];
  const missing = required.filter((value) => !desktop.includes(value));
  if (missing.length > 0)
  failures.push(`desktop missing: ${missing.join(", ")}`);
  const detail = "desktop entry, icon, and AppStream XML validated";
  return {
    detail: failures.length === 0 ? detail : failures.join("; "),
    name: "Packaging metadata",
    ok: failures.length === 0,
  };
}

function validateXmlFile(path: string): string[] {
  const absolutePath = join(PROJECT_ROOT, path);
  const error = validateXmlText(readFileSync(absolutePath, "utf8"));
  return error === null ? [] : [`${path}: ${error}`];
}

function checkShellScripts(): Check {
  const directory = join(PROJECT_ROOT, "scripts");
  const scripts = readdirSync(directory)
  .filter((name) => name.endsWith(".sh"))
  .map((name) => join(directory, name))
  .sort();
  const failures: string[] = [];
  for (const script of scripts) {
    const checked = run(["bash", "-n", script]);
    if (checked.status !== 0) {
      failures.push(`${relativePath(script)}: ${commandDetail(checked)}`);
    }
  }
  return {
    detail:
    failures.length === 0
    ? `validated ${scripts.length} scripts`
    : failures.join("; "),
    name: "Shell syntax",
    ok: failures.length === 0,
  };
}

function checkNativeTools(): Check[] {
  const cargo = findExecutable("cargo");
  if (cargo === null) {
    return [
      cargoSkipped("Cargo check"),
      cargoSkipped("Cargo tests"),
      checkQmlLint(),
    ];
  }
  const checked = run([cargo, "check", "--workspace"]);
  const tested = run([cargo, "test", "-p", "ticket-core"]);
  return [
    {
      detail: commandDetail(checked),
      name: "Cargo check",
      ok: checked.status === 0,
    },
    {
      detail: commandDetail(tested),
      name: "Cargo tests",
      ok: tested.status === 0,
    },
    checkQmlLint(),
  ];
}

function cargoSkipped(name: string): Check {
  return {
    detail: "cargo is not installed in this environment",
    name,
    ok: true,
    skipped: true,
  };
}

function checkQmlLint(): Check {
  const qmllint = findExecutable("qmllint") ?? fedoraQmlLint();
  if (qmllint === null) {
    return {
      detail: "qmllint is not installed in this environment",
      name: "QML lint",
      ok: true,
      skipped: true,
    };
  }
  const command = [qmllint];
  for (const modulePath of qmlModulePaths()) {
    command.push("-I", relative(PROJECT_ROOT, modulePath));
  }
  const linted = run([...command, "crates/ticket-kde/src/qml/app/main.qml"]);
  return {
    detail: commandDetail(linted),
    name: "QML lint",
    ok: linted.status === 0,
  };
}

function fedoraQmlLint(): string | null {
  const path = "/usr/lib64/qt6/bin/qmllint";
  return existsSync(path) ? path : null;
}

function qmlModulePaths(): string[] {
  const buildRoot = join(PROJECT_ROOT, "target/debug/build");
  if (!existsSync(buildRoot)) return [];
  return readdirSync(buildRoot)
  .filter((name) => name.startsWith("ticket-kde-"))
  .map((name) => join(buildRoot, name, "out/qt-build-utils/qml_modules"))
  .filter((path) => existsSync(path))
  .sort();
}

export {
  checkNativeTools,
  checkPackagingMetadata,
  checkRequiredFiles,
  checkShellScripts,
  checkToml,
};
