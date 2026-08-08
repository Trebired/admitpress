import { BINARY_NAME, runCargo } from "./project.js";

if (typeof process.getuid === "function" && process.getuid() === 0) {
  console.error(
    "Do not run the application with sudo. Run it as your normal desktop user:",
  );
  console.error("  bun run dev");
  process.exit(1);
}

process.env.TICKET_APP_DEVELOPMENT_MODE_DEFAULT ??= "true";

runCargo(["run", "-p", "ticket-kde", "--bin", BINARY_NAME, "--", ...Bun.argv.slice(2)]);
