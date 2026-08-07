import { runVerification } from "../verify/report.js";
import { runCargo, runOrExit, runTypecheck } from "./project.js";

runTypecheck();
runOrExit("bun", ["test", "tools/reports"]);
runCargo(["test", "-p", "ticket-core"]);
runCargo([
    "clippy",
    "-p",
    "ticket-core",
    "--all-targets",
    "--",
    "-D",
    "warnings",
]);
process.exit(runVerification([]));
