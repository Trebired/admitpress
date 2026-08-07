import { readFileSync } from "node:fs";
import { join } from "node:path";
import { PROJECT_ROOT, type Check, walkFiles } from "./common.js";

function snakeToCamel(value: string): string {
  const [head = "", ...tail] = value.split("_");
  return head + tail.map(capitalize).join("");
}

function capitalize(value: string): string {
  return value.slice(0, 1).toUpperCase() + value.slice(1);
}

function checkQmlBackendContract(): Check {
  const backend = readFileSync(
    join(PROJECT_ROOT, "crates/ticket-kde/src/backend.rs"),
    "utf8",
  );
  const qml = qmlFiles()
  .map((path) => readFileSync(path, "utf8"))
  .join("\n");
  const bridge = backendBridgeSection(backend);
  const declared = new Set(
    [...bridge.matchAll(/\bfn\s+([a-z][a-z0-9_]*)\s*\(/g)].map((match) =>
        snakeToCamel(match[1] ?? ""),
      ),
    );
    const called = new Set(
      [...qml.matchAll(/\bbackend\.([A-Za-z][A-Za-z0-9_]*)\s*\(/g)].map(
          (match) => match[1] ?? "",
        ),
      );
      const missing = [...called].filter((name) => !declared.has(name)).sort();
      const detail = `validated ${called.size} QML calls against ${declared.size} invokables`;
      return {
        detail:
        missing.length === 0 ? detail : `undeclared calls: ${missing.join(", ")}`,
        name: "QML backend contract",
        ok: missing.length === 0,
      };
    }

    function backendBridgeSection(backend: string): string {
      const boundary = "pub struct TicketBackendRust";
      const index = backend.indexOf(boundary);
      return index < 0 ? backend : backend.slice(0, index);
    }

    function checkQmlModuleContract(): Check {
      const build = readFileSync(
        join(PROJECT_ROOT, "crates/ticket-kde/build.rs"),
        "utf8",
      );
      const main = readFileSync(
        join(PROJECT_ROOT, "crates/ticket-kde/src/main.rs"),
        "utf8",
      );
      const qml = readFileSync(
        join(PROJECT_ROOT, "crates/ticket-kde/src/qml/app/main.qml"),
        "utf8",
      );
      const uri = qmlModuleUri(build);
      if (uri === null) {
        return {
          detail: "QML module URI is missing from build.rs",
          name: "QML module contract",
          ok: false,
        };
      }
      const failures = qmlModuleFailures(uri, main, qml);
      return {
        detail:
        failures.length === 0
        ? `URI ${uri} matches QML import and resource path`
        : failures.join("; "),
        name: "QML module contract",
        ok: failures.length === 0,
      };
    }

    function qmlModuleUri(build: string): string | null {
      const literal = /QmlModule::new\("([^"]+)"\)/.exec(build);
    if (literal) return literal[1] ?? null;

    const constants = build.matchAll(
    /const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]+)"/g,
      );
      for (const constant of constants) {
        if (build.includes(`QmlModule::new(${constant[1]})`)) {
          return constant[2] ?? null;
        }
      }

      return null;
    }

    function qmlModuleFailures(uri: string, main: string, qml: string): string[] {
      const failures: string[] = [];
      const resource = `qrc:/qt/qml/${uri.replaceAll(".", "/")}/src/qml/app/main.qml`;
      if (!qml.includes(`import ${uri}`))
      failures.push(`main.qml does not import ${uri}`);
      if (!main.includes(resource))
      failures.push(`main.rs does not load ${resource}`);
      return failures;
    }

    function qmlFiles(): string[] {
      const root = join(PROJECT_ROOT, "crates/ticket-kde/src/qml");
      return walkFiles(root).filter((path) => path.endsWith(".qml"));
    }

    export { checkQmlBackendContract, checkQmlModuleContract };
