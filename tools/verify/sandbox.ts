import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { PROJECT_ROOT } from "../native/project.js";
import type { Check } from "./common.js";

const TOOLING_DIRECTORIES = ["tools/native", "tools/project"];

const ESCAPES: [RegExp, string][] = [
  [/\bhomedir\s*\(/, "homedir()"],
    [/XDG_DATA_HOME/, "XDG_DATA_HOME"],
    [/XDG_CONFIG_HOME/, "XDG_CONFIG_HOME"],
    [/["'`]\/home\//, "an absolute /home path"],
        [/\.local\/share/, ".local/share"],
        [/update-desktop-database/, "update-desktop-database"],
        [/gtk-update-icon-cache/, "gtk-update-icon-cache"],
        [/kbuildsycoca/, "kbuildsycoca"],
        ];

        function toolingFiles(): string[] {
        const files: string[] = [];
        for (const directory of TOOLING_DIRECTORIES) {
        const root = join(PROJECT_ROOT, directory);
        for (const entry of readdirSync(root, { withFileTypes: true })) {
        if (entry.isFile() && entry.name.endsWith(".ts")) {
        files.push(join(directory, entry.name));
        }
        }
        }
        return files.sort();
        }

        function checkDevelopmentSandbox(): Check {
        const failures: string[] = [];
        const files = toolingFiles();

        for (const file of files) {
        const text = readFileSync(join(PROJECT_ROOT, file), "utf8");
        for (const [pattern, label] of ESCAPES) {
        if (pattern.test(text)) {
        failures.push(`${file} reaches outside the repository through ${label}`);
        }
        }
        }

        return {
        detail:
        failures.length === 0
        ? `${files.length} tooling files stay inside the repository`
        : failures.join("; "),
        name: "Development sandbox",
        ok: failures.length === 0,
        };
        }

        export { checkDevelopmentSandbox };
