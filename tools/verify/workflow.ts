import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import {
  PROJECT_ROOT,
  type Check,
  createTemporaryDirectory,
  removeDirectory,
} from "./common.js";

const RECORDED_AT = "2026-08-02T22:44:00+02:00";
const LOCAL_DATE = "2026-08-02";
const TICKET_ID = "ticket-1";
const SUBTICKETS: readonly [string, number][] = [
  ["standard", 1000],
  ["reduced", 700],
  ["guest", 300],
];

type WorkflowState = {
  allocated: number[];
  historyAfter: number;
  historyBefore: number;
  nextNumbers: Map<string, number>;
  tables: Set<string>;
};

type SqlValue = number | string;

function checkMigrationAndWorkflow(): Check {
  const migration = readFileSync(
    join(PROJECT_ROOT, "crates/ticket-core/migrations/0001_initial.sql"),
    "utf8",
  );
  const state = initialState();
  const directory = createTemporaryDirectory("ticket-verify-");
  try {
    const database = openDatabase(directory, migration);
    try {
      insertTicket(database);
      reservePrintRecords(database, state);
      completeRecords(database, state);
      adjustCounter(database, state);
      state.tables = databaseTables(database);
    } finally {
      database.close();
    }
  } catch (error) {
    return { detail: errorDetail(error), name: "SQLite workflow", ok: false };
  } finally {
    removeDirectory(directory);
  }
  return workflowCheck(state);
}

function initialState(): WorkflowState {
  return {
    allocated: [],
    historyAfter: 0,
    historyBefore: 0,
    nextNumbers: new Map(),
    tables: new Set(),
  };
}

function openDatabase(directory: string, migration: string): Database {
  const database = new Database(join(directory, "application.sqlite"));
  database.run("PRAGMA foreign_keys = ON");
  database.exec(migration);
  return database;
}

function insertTicket(database: Database): void {
  database.query(ticketInsertSql()).run(...ticketValues());
  SUBTICKETS.forEach(([subticketId, amountMinor], sortOrder) => {
      database
      .query(subticketInsertSql())
      .run(...subticketValues(subticketId, amountMinor, sortOrder));
  });
}

function ticketInsertSql(): string {
  return `
  INSERT INTO tickets (
  id, name, width_mm, height_mm, dpi, number_start, number_increment,
  number_padding, number_prefix, number_suffix, recorded_at, updated_at
  ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
  `;
}

function ticketValues(): SqlValue[] {
  return [
    TICKET_ID,
    "Entrance",
    80.0,
    50.0,
    203,
    1,
    1,
    4,
    "",
    "",
    RECORDED_AT,
    RECORDED_AT,
  ];
}

function subticketInsertSql(): string {
  return `
  INSERT INTO subtickets (
  id, ticket_id, name, amount_minor, currency, display_label,
  enabled, sort_order, recorded_at, updated_at
  ) VALUES (?, ?, ?, ?, '', ?, 1, ?, ?, ?)
  `;
}

function subticketValues(
  subticketId: string,
  amountMinor: number,
  sortOrder: number,
): SqlValue[] {
  return [
    subticketId,
    TICKET_ID,
    subticketId,
    amountMinor,
    subticketId,
    sortOrder,
    RECORDED_AT,
    RECORDED_AT,
  ];
}

function reservePrintRecords(database: Database, state: WorkflowState): void {
  database.run("BEGIN IMMEDIATE");
  SUBTICKETS.forEach(([subticketId], index) => {
      reserveCounter(database, subticketId);
      const nextNumber = readNextNumber(database, subticketId);
      state.allocated.push(nextNumber);
      insertPrintRecord(database, index + 1, subticketId, nextNumber);
      state.nextNumbers.set(subticketId, nextNumber + 1);
  });
  for (const [subticketId, nextNumber] of state.nextNumbers) {
    updateNextNumber(database, subticketId, nextNumber);
  }
  database.run("COMMIT");
}

function reserveCounter(database: Database, subticketId: string): void {
  database
  .query(
    `
    INSERT INTO daily_counters (
    ticket_id, subticket_id, local_date, next_number, recorded_at, updated_at
    ) VALUES (?, ?, ?, 1, ?, ?)
    `,
  )
  .run(TICKET_ID, subticketId, LOCAL_DATE, RECORDED_AT, RECORDED_AT);
}

function readNextNumber(database: Database, subticketId: string): number {
  const row = database
  .query(counterSelectSql())
  .get(TICKET_ID, subticketId, LOCAL_DATE) as {
    next_number?: unknown;
  } | null;
  return Number(row?.next_number ?? 0);
}

function counterSelectSql(): string {
  return `
  SELECT next_number FROM daily_counters
  WHERE ticket_id = ? AND subticket_id = ? AND local_date = ?
  `;
}

function insertPrintRecord(
  database: Database,
  index: number,
  subticketId: string,
  nextNumber: number,
): void {
  database
  .query(
    `
    INSERT INTO print_records (
    id, ticket_id, subticket_id, local_date, allocated_number,
    status, is_bulk, recorded_at
    ) VALUES (?, ?, ?, ?, ?, 'pending', 0, ?)
    `,
  )
  .run(
    `print-${index}`,
    TICKET_ID,
    subticketId,
    LOCAL_DATE,
    nextNumber,
    RECORDED_AT,
  );
}

function updateNextNumber(
  database: Database,
  subticketId: string,
  nextNumber: number,
): void {
  database
  .query(
    `
    UPDATE daily_counters SET next_number = ?, updated_at = ?
    WHERE ticket_id = ? AND subticket_id = ? AND local_date = ?
    `,
  )
  .run(nextNumber, RECORDED_AT, TICKET_ID, subticketId, LOCAL_DATE);
}

function completeRecords(database: Database, state: WorkflowState): void {
  database
  .query(
    "UPDATE print_records SET status = 'completed', started_at = ?, ended_at = ?",
  )
  .run(RECORDED_AT, RECORDED_AT);
  state.historyBefore = countRows(database, "print_records");
}

function adjustCounter(database: Database, state: WorkflowState): void {
  database.run("BEGIN IMMEDIATE");
  const first = SUBTICKETS[0];
  if (first === undefined) throw new Error("Missing verification subticket");
  const [subticketId] = first;
  const previous = readNextNumber(database, subticketId);
  updateNextNumber(database, subticketId, 2);
  insertAdjustment(database, subticketId, previous);
  database.run("COMMIT");
  state.historyAfter = countRows(database, "print_records");
}

function insertAdjustment(
  database: Database,
  subticketId: string,
  previous: number,
): void {
  database
  .query(
    `
    INSERT INTO counter_adjustments (
    id, ticket_id, subticket_id, local_date, previous_next_number,
    new_next_number, reason, recorded_at
    ) VALUES ('adjustment-1', ?, ?, ?, ?, 2, 'Manual correction', ?)
    `,
  )
  .run(TICKET_ID, subticketId, LOCAL_DATE, previous, RECORDED_AT);
}

function countRows(database: Database, table: string): number {
  const row = database
  .query(`SELECT COUNT(*) AS count FROM ${table}`)
  .get() as {
    count?: unknown;
  } | null;
  return Number(row?.count ?? 0);
}

function databaseTables(database: Database): Set<string> {
  const rows = database
  .query("SELECT name FROM sqlite_master WHERE type = 'table'")
  .all() as { name?: unknown }[];
  return new Set(
    rows
    .map((row) => String(row.name ?? ""))
    .filter((name) => name.length > 0 && !name.startsWith("sqlite_")),
  );
}

function workflowCheck(state: WorkflowState): Check {
  const expectedTables = [
    "tickets",
    "ticket_fields",
    "subtickets",
    "daily_counters",
    "print_records",
    "counter_adjustments",
    "app_settings",
  ];
  const ok = workflowOk(state, expectedTables);
  return {
    detail: workflowDetail(state),
    name: "SQLite workflow",
    ok,
  };
}

function workflowOk(state: WorkflowState, expectedTables: string[]): boolean {
  return (
    valuesEqual(state.allocated, [1, 1, 1]) &&
      valuesEqual([...state.nextNumbers.values()].sort(), [2, 2, 2]) &&
      state.historyBefore === 3 &&
      state.historyAfter === 3 &&
      expectedTables.every((table) => state.tables.has(table))
  );
}

function workflowDetail(state: WorkflowState): string {
  const next = [...state.nextNumbers.values()].sort();
  return `tables=${state.tables.size}, allocated=${state.allocated}, next=${next}, history=${state.historyAfter}`;
}

function valuesEqual(left: number[], right: number[]): boolean {
  return (
    left.length === right.length &&
      left.every((value, index) => value === right[index])
  );
}

function errorDetail(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

export { checkMigrationAndWorkflow };
