import { formatMoney } from "./money.js";
import type { DaySalesSummary } from "./sales.js";

function renderTable(summaries: readonly DaySalesSummary[]): string {
  if (summaries.length === 0) return "No sales recorded.";

  const lines: string[] = [];
  for (const summary of summaries) {
    lines.push(`${summary.localDate}  ${summary.ticketCount} sold  ${formatMoney(summary.totalMinor, summary.currency)}`);
    for (const subticket of summary.subtickets) {
      const label = subticket.displayLabel || subticket.name;
      const price = formatMoney(subticket.amountMinor, subticket.currency);
      const total = formatMoney(subticket.totalMinor, subticket.currency);
      lines.push(`  ${label} x${subticket.quantity} @ ${price} = ${total}`);
    }
  }
  return lines.join("\n");
}

function csvField(value: string | number): string {
  const text = String(value);
  if (!/[",\n]/.test(text)) return text;
      return `"${text.replace(/"/g, '""')}"`;
          }

          function renderCsv(summaries: readonly DaySalesSummary[]): string {
          const header = ["local_date", "subticket_id", "name", "quantity", "amount_minor", "total_minor", "currency"];
          const rows = summaries.flatMap((summary) =>
          summary.subtickets.map((subticket) => [
          summary.localDate,
          subticket.subticketId,
          subticket.name,
          subticket.quantity,
          subticket.amountMinor,
          subticket.totalMinor,
          subticket.currency,
          ]),
          );
          return [header, ...rows].map((row) => row.map(csvField).join(",")).join("\n");
          }

          function renderJson(summaries: readonly DaySalesSummary[]): string {
          return JSON.stringify(summaries, null, 2);
          }

          export { renderCsv, renderJson, renderTable };
