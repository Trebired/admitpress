import QtQuick
import "../components"

QtObject {
  id: activity

  required property var root
  required property var page

  function printOperations() {
    return page.summary && page.summary.date_print_operations
    ? page.summary.date_print_operations
    : []
  }

  function counterAdjustments() {
    return page.summary && page.summary.date_counter_adjustments
    ? page.summary.date_counter_adjustments
    : []
  }

  function subticketLabel(subticketId) {
    const subtickets = page.summary && page.summary.subtickets ? page.summary.subtickets : []
    for (let subticket of subtickets) {
      if (subticket.id !== subticketId) continue
      return subticket.display_label
      || subticket.name
      || MoneyFormat.format(subticket.amount_minor, subticket.currency)
    }
    return ""
  }

  function printedOutput(operation) {
    return !!operation.rendered_output_path
  }

  function printTitle(operation) {
    if (operation.status === "failed") return LocaleText.tr("Print failed")
    if (!printedOutput(operation)) return LocaleText.tr("Recorded")
    return LocaleText.tr("Printed")
  }

  function printIconRole(operation) {
    if (operation.status === "failed") return "error"
    if (!printedOutput(operation)) return "recorded"
    return "ok"
  }

  function printSubtitle(operation) {
    const parts = [TextFormat.ticketCount(operation.ticket_count)]
    if (operation.status === "queued") parts.push(LocaleText.tr("Queued"))
    if (operation.operation_label) parts.push(operation.operation_label)
    return parts.join("  ")
  }

  function printFilterKind(operation) {
    if (operation.status === "failed") return "failed"
    return printedOutput(operation) ? "printed" : "recorded"
  }

  function printEntry(operation) {
    return {
      kind: "print",
      filterKind: printFilterKind(operation),
      recorded_at: operation.recorded_at || "",
      operation: operation,
      iconRole: printIconRole(operation),
      number: operation.print_number,
      title: printTitle(operation),
      subtitle: printSubtitle(operation)
    }
  }

  function counterEntry(adjustment) {
    const priceLabel = subticketLabel(adjustment.subticket_id)
    const change = TextFormat.displayTicketNumber(adjustment.previous_next_number, page.ticketNumberPadding())
    + " → "
    + TextFormat.displayTicketNumber(adjustment.new_next_number, page.ticketNumberPadding())
    const parts = []
    if (priceLabel) parts.push(priceLabel)
    parts.push(change)
    if (adjustment.reason) parts.push(adjustment.reason)
    return {
      kind: "counter",
      filterKind: "counter",
      recorded_at: adjustment.recorded_at || "",
      operation: null,
      iconRole: "number",
      number: adjustment.new_next_number,
      title: LocaleText.tr("Counter adjusted"),
      subtitle: parts.join("  ")
    }
  }

  function filterChoices() {
    return [
      { text: LocaleText.tr("All activity"), kind: "" },
      { text: LocaleText.tr("Printed"), kind: "printed" },
      { text: LocaleText.tr("Recorded"), kind: "recorded" },
      { text: LocaleText.tr("Print failed"), kind: "failed" },
      { text: LocaleText.tr("Counter adjusted"), kind: "counter" }
    ]
  }

  function matchesSearch(entry, term) {
    if (!term) return true
    const haystack = (entry.title + " " + entry.subtitle).toLowerCase()
    return haystack.indexOf(term) >= 0
  }

  function entries() {
    const kind = page.activityFilter || ""
    const term = String(page.activitySearch || "").trim().toLowerCase()
    const merged = []
    for (let operation of printOperations()) merged.push(printEntry(operation))
    for (let adjustment of counterAdjustments()) merged.push(counterEntry(adjustment))
    const filtered = merged.filter(function(entry) {
        if (kind && entry.filterKind !== kind) return false
        return matchesSearch(entry, term)
    })
    filtered.sort(function(left, right) {
        return String(right.recorded_at).localeCompare(String(left.recorded_at))
    })
    return filtered
  }

  function hasEntries() {
    return printOperations().length > 0 || counterAdjustments().length > 0
  }
}
