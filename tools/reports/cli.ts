import { Database } from "bun:sqlite";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { PROJECT_ROOT } from "../native/project.js";
import { renderCsv, renderJson, renderTable } from "./format.js";
import { listDaySales, listTickets } from "./sales.js";

type OutputFormat = "csv" | "json" | "table";

type CliOptions = {
  databasePath: string;
  date: string;
  format: OutputFormat;
  limit: number;
  listTickets: boolean;
  ticketId: string | undefined;
};

const USAGE = `Usage: bun run report -- --ticket <id> [options]

Options:
--ticket <id>      Ticket id to summarize (required unless --list-tickets)
--list-tickets      List available ticket ids and exit
--date <yyyy-mm-dd>  Include this date in the output even with zero sales
--limit <n>         Maximum number of days to include (default: 30)
--format <format>    table (default), csv, or json
--db <path>          Path to the SQLite database (default: data/application.sqlite)`;

function parseFormat(value: string): OutputFormat {
  if (value === "csv" || value === "json" || value === "table") return value;
  throw new Error(`Unknown --format value: ${value}`);
}

function parseOptions(argv: readonly string[]): CliOptions {
  const options: CliOptions = {
    databasePath: join(PROJECT_ROOT, "data", "application.sqlite"),
    date: "",
    format: "table",
    limit: 30,
    listTickets: false,
    ticketId: undefined,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const flag = argv[index];
    const next = (): string => {
      const value = argv[index + 1];
      if (value === undefined) throw new Error(`Missing value for ${flag}`);
      index += 1;
      return value;
    };

    if (flag === "--ticket") options.ticketId = next();
    else if (flag === "--list-tickets") options.listTickets = true;
    else if (flag === "--date") options.date = next();
    else if (flag === "--limit") options.limit = Number.parseInt(next(), 10);
    else if (flag === "--format") options.format = parseFormat(next());
    else if (flag === "--db") options.databasePath = next();
    else if (flag === "--help") {
      console.log(USAGE);
      process.exit(0);
    } else throw new Error(`Unknown option: ${flag}`);
  }

  return options;
}

function render(summaries: ReturnType<typeof listDaySales>, format: OutputFormat): string {
  if (format === "csv") return renderCsv(summaries);
  if (format === "json") return renderJson(summaries);
  return renderTable(summaries);
}

function main(): void {
  const options = parseOptions(process.argv.slice(2));

  if (!existsSync(options.databasePath)) {
    console.error(`Database not found at ${options.databasePath}`);
    process.exit(1);
  }

  const db = new Database(options.databasePath, { readonly: true });

  if (options.listTickets) {
    for (const ticket of listTickets(db)) console.log(`${ticket.id}  ${ticket.name}`);
    return;
  }

  if (!options.ticketId) {
    console.error("Missing required --ticket <id>. Pass --list-tickets to see options.\n");
    console.error(USAGE);
    process.exit(1);
  }

  const summaries = listDaySales(db, options.ticketId, options.date, options.limit);
  console.log(render(summaries, options.format));
}

main();
