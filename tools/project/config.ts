import { readFileSync } from "node:fs";
import { join } from "node:path";

type ProjectPackageData = {
  config?: {
    app?: Record<string, unknown>;
  };
};

function projectConfigString(projectRoot: string, key: string): string {
  const packageData = JSON.parse(
    readFileSync(join(projectRoot, "package.json"), "utf8"),
  ) as ProjectPackageData;
  const value = packageData.config?.app?.[key];
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new Error(`package.json#config.app.${key} must be set`);
  }
  return value.trim();
}

export { projectConfigString };
