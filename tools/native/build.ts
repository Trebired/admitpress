import { runCargo, runTypecheck } from "./project.js";

runTypecheck();
runCargo(["build", "--workspace"]);
