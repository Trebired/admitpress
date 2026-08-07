import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { beforeEach, describe, expect, test } from "bun:test";
import { formatMoney } from "./money.js";
import { listDaySales, listTickets } from "./sales.js";

const MIGRATION_PATH = join(
  import.meta.dir,
  "../../crates/ticket-core/migrations/0001_initial.sql",
);

function createSchema(db: Database): void {
  const migration = readFileSync(MIGRATION_PATH, "utf8");
  for (const statement of migration.split(";")) {
    const trimmed = statement.trim();
    if (trimmed.length > 0) db.run(trimmed);
  }
}

function seedTicket(db: Database, id: string): void {
  db.run(
    `INSERT INTO tickets (id, name, width_mm, height_mm, number_start, number_increment,
    number_padding, number_prefix, number_suffix, recorded_at, updated_at)
    VALUES (?, 'Concert', 80, 50, 1, 1, 0, '', '', '2026-01-01', '2026-01-01')`,
    [id],
  );
}

function seedSubticket(
  db: Database,
  id: string,
  ticketId: string,
  name: string,
  amountMinor: number,
  currency: string,
  sortOrder: number,
): void {
  db.run(
    `INSERT INTO subtickets (id, ticket_id, name, amount_minor, currency, display_label,
    enabled, sort_order, recorded_at, updated_at)
    VALUES (?, ?, ?, ?, ?, ?, 1, ?, '2026-01-01', '2026-01-01')`,
    [id, ticketId, name, amountMinor, currency, name, sortOrder],
  );
}

function seedPrintRecord(
  db: Database,
  ticketId: string,
  subticketId: string,
  localDate: string,
  allocatedNumber: number,
  status: string,
): void {
  db.run(
    `INSERT INTO print_records (id, ticket_id, subticket_id, local_date, allocated_number,
    status, is_bulk, recorded_at)
    VALUES (?, ?, ?, ?, ?, ?, 0, ?)`,
    [
      `${subticketId}-${localDate}-${allocatedNumber}`,
      ticketId,
      subticketId,
      localDate,
      allocatedNumber,
      status,
      `${localDate}T10:00:00`,
    ],
  );
}

const ticketId = "ticket-1";
const adultId = "sub-adult";
const childId = "sub-child";

function seedFixtures(): Database {
  const db = new Database(":memory:");
  createSchema(db);
  seedTicket(db, ticketId);
  seedSubticket(db, adultId, ticketId, "Adult", 1000, "CZK", 0);
  seedSubticket(db, childId, ticketId, "Child", 500, "CZK", 1);

  seedPrintRecord(db, ticketId, adultId, "2026-01-02", 1, "completed");
  seedPrintRecord(db, ticketId, adultId, "2026-01-02", 2, "queued");
  seedPrintRecord(db, ticketId, childId, "2026-01-02", 1, "completed");
  seedPrintRecord(db, ticketId, adultId, "2026-01-02", 3, "failed");
  seedPrintRecord(db, ticketId, adultId, "2026-01-01", 1, "completed");
  return db;
}

describe("listDaySales", () => {
    let db: Database;

    beforeEach(() => {
        db = seedFixtures();
    });

    test("aggregates quantity and revenue per subticket and date", () => {
        const summaries = listDaySales(db, ticketId, "", 30);

        expect(summaries).toHaveLength(2);
        expect(summaries[0]?.localDate).toBe("2026-01-02");
        expect(summaries[0]?.ticketCount).toBe(3);
        expect(summaries[0]?.totalMinor).toBe(2500);

        const adult = summaries[0]?.subtickets.find((entry) => entry.subticketId === adultId);
        expect(adult?.quantity).toBe(2);
        expect(adult?.totalMinor).toBe(2000);
    });

    test("excludes failed print records from totals", () => {
        const summaries = listDaySales(db, ticketId, "", 30);
        const total = summaries.reduce((sum, summary) => sum + summary.ticketCount, 0);
        expect(total).toBe(4);
    });

    test("orders days newest first", () => {
        const summaries = listDaySales(db, ticketId, "", 30);
        expect(summaries.map((summary) => summary.localDate)).toEqual(["2026-01-02", "2026-01-01"]);
    });

    test("injects a zero-sales placeholder for a requested date with no records", () => {
        const summaries = listDaySales(db, ticketId, "2026-01-05", 30);
        expect(summaries[0]?.localDate).toBe("2026-01-05");
        expect(summaries[0]?.ticketCount).toBe(0);
        expect(summaries[0]?.subtickets).toHaveLength(0);
    });

    test("truncates to the requested limit", () => {
        const summaries = listDaySales(db, ticketId, "", 1);
        expect(summaries).toHaveLength(1);
        expect(summaries[0]?.localDate).toBe("2026-01-02");
    });

});

describe("listTickets", () => {
    let db: Database;

    beforeEach(() => {
        db = seedFixtures();
    });

    test("lists tickets by name", () => {
        seedTicket(db, "ticket-0");
        db.run("UPDATE tickets SET name = 'Aardvark Show' WHERE id = 'ticket-0'");
        const tickets = listTickets(db);
        expect(tickets.map((ticket) => ticket.id)).toEqual(["ticket-0", ticketId]);
    });
});

describe("formatMoney", () => {
    test("formats a whole amount without a currency", () => {
        expect(formatMoney(1_000, "")).toBe("10");
    });

    test("formats a fractional amount with a known currency suffix", () => {
        expect(formatMoney(123, "CZK")).toBe("1,23 Kč");
    });

    test("formats EUR with a suffix and USD with a prefix", () => {
        expect(formatMoney(500, "EUR")).toBe("5 €");
        expect(formatMoney(599, "USD")).toBe("$5,99");
    });

    test("falls back to the raw currency code when unknown", () => {
        expect(formatMoney(100, "GBP")).toBe("1 GBP");
    });

    test("formats negative amounts with a leading sign", () => {
        expect(formatMoney(-150, "CZK")).toBe("-1,50 Kč");
    });
});
