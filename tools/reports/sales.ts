import type { Database } from "bun:sqlite";

type DaySubticketSales = {
  subticketId: string;
  name: string;
  displayLabel: string;
  amountMinor: number;
  currency: string;
  quantity: number;
  totalMinor: number;
};

type DaySalesSummary = {
  localDate: string;
  ticketCount: number;
  totalMinor: number;
  currency: string;
  subtickets: DaySubticketSales[];
};

type TicketOption = {
  id: string;
  name: string;
};

type DaySalesRow = {
  local_date: string;
  subticket_id: string;
  name: string;
  display_label: string;
  amount_minor: number;
  currency: string;
  quantity: number;
  total_minor: number;
};

const DAY_SALES_QUERY = `
SELECT
records.local_date AS local_date,
subtickets.id AS subticket_id,
subtickets.name AS name,
subtickets.display_label AS display_label,
subtickets.amount_minor AS amount_minor,
subtickets.currency AS currency,
COUNT(*) AS quantity,
COALESCE(SUM(subtickets.amount_minor), 0) AS total_minor
FROM print_records records
JOIN subtickets ON subtickets.id = records.subticket_id
WHERE records.ticket_id = ?
AND records.status IN ('completed', 'queued')
GROUP BY
records.local_date,
subtickets.id,
subtickets.name,
subtickets.display_label,
subtickets.amount_minor,
subtickets.currency,
subtickets.sort_order
ORDER BY records.local_date DESC, subtickets.sort_order, subtickets.recorded_at
`;

function emptySummary(localDate: string): DaySalesSummary {
  return {
    currency: "",
    localDate,
    subtickets: [],
    ticketCount: 0,
    totalMinor: 0,
  };
}

function appendRow(summaries: DaySalesSummary[], row: DaySalesRow): void {
  const last = summaries[summaries.length - 1];
  const summary = last && last.localDate === row.local_date ? last : emptySummary(row.local_date);
  if (summary !== last) summaries.push(summary);

  if (summary.ticketCount === 0) summary.currency = row.currency;
  summary.ticketCount += row.quantity;
  summary.totalMinor += row.total_minor;
  summary.subtickets.push({
      amountMinor: row.amount_minor,
      currency: row.currency,
      displayLabel: row.display_label,
      name: row.name,
      quantity: row.quantity,
      subticketId: row.subticket_id,
      totalMinor: row.total_minor,
  });
}

function withIncludedDate(
  summaries: DaySalesSummary[],
  includeDate: string,
): DaySalesSummary[] {
  const trimmed = includeDate.trim();
  if (trimmed === "") return summaries;
  if (summaries.some((summary) => summary.localDate === trimmed)) return summaries;
  return [emptySummary(trimmed), ...summaries];
}

function listDaySales(
  db: Database,
  ticketId: string,
  includeDate: string,
  limit: number,
): DaySalesSummary[] {
  const rows = db.query<DaySalesRow, [string]>(DAY_SALES_QUERY).all(ticketId);
  const summaries: DaySalesSummary[] = [];
  for (const row of rows) appendRow(summaries, row);

  const withRequestedDate = withIncludedDate(summaries, includeDate);
  return withRequestedDate.slice(0, Math.max(limit, 0));
}

function listTickets(db: Database): TicketOption[] {
  return db
  .query<TicketOption, []>("SELECT id, name FROM tickets ORDER BY name")
  .all();
}

export { listDaySales, listTickets, type DaySalesSummary, type DaySubticketSales, type TicketOption };
