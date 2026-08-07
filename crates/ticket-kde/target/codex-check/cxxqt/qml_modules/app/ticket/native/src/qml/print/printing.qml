import QtQuick
import "../components"

QtObject {
  id: logic

  required property var root
  required property var backend
  required property var page

  property QtObject days: PrintDays {
    root: logic.root
    backend: logic.backend
    page: logic.page
  }
  property bool submitInFlight: false

  function ticketNumberPadding() {
    return page.summary && page.summary.ticket ? page.summary.ticket.number_padding : 0
  }

  function displayTicketNumber(value) {
    return TextFormat.displayTicketNumber(value, ticketNumberPadding())
  }

  function quantityRows() {
    return page.quantityModel
  }

  function syncQuantityModel() {
    const previous = {}
    const model = quantityRows()
    for (let index = 0; index < model.count; ++index) {
      const row = model.get(index)
      previous[row.subticket_id] = Number(row.quantity || 0)
    }
    model.clear()
    const subtickets = page.summary && page.summary.subtickets ? page.summary.subtickets : []
    for (let subticket of subtickets) {
      model.append({
          subticket_id: subticket.id,
          amount_minor: subticket.amount_minor,
          currency: subticket.currency || "",
          label: root.formatMoney(subticket.amount_minor, subticket.currency),
          display_label: subticket.display_label || subticket.name || "",
          current_next: subticket.next_number_today,
          next_number_today: subticket.next_number_today,
          quantity: previous[subticket.id] || 0
      })
    }
    if (page.activeQuantityIndex < 0 || page.activeQuantityIndex >= model.count) {
      page.activeQuantityIndex = model.count > 0 ? 0 : -1
    }
    page.selectionRevision += 1
  }

  function setSelectionQuantity(index, quantity) {
    const model = quantityRows()
    if (index < 0 || index >= model.count) return
    if (page.setActiveQuantityIndex) page.setActiveQuantityIndex(index)
    const normalized = Math.max(0, Math.min(10000, Number(quantity || 0)))
    model.setProperty(index, "quantity", normalized)
    page.selectionRevision += 1
  }

  function selectedRows() {
    page.selectionRevision
    const rows = []
    const model = quantityRows()
    for (let index = 0; index < model.count; ++index) {
      const row = model.get(index)
      if (Number(row.quantity || 0) > 0) rows.push(row)
    }
    return rows
  }

  function selectedRequests() {
    const requests = []
    for (let row of selectedRows()) {
      requests.push({ subticket_id: row.subticket_id, quantity: Number(row.quantity || 0) })
    }
    return requests
  }

  function selectedCount() {
    let count = 0
    for (let row of selectedRows()) count += Number(row.quantity || 0)
    return count
  }

  function totalMinor() {
    let total = 0
    for (let row of selectedRows()) {
      total += Number(row.quantity || 0) * Number(row.amount_minor || 0)
    }
    return total
  }

  function selectedCurrency() {
    const rows = selectedRows()
    return rows.length > 0 ? rows[0].currency || "" : ""
  }

  function selectionText() {
    const count = selectedCount()
    if (count === 0) return LocaleText.tr("No tickets selected")
    return TextFormat.ticketCount(count) + ", " + root.formatMoney(totalMinor(), selectedCurrency())
  }

  function selectionSummaryText() {
    page.selectionRevision
    return selectedCount() === 0 ? "No tickets selected" : selectionText()
  }

  function canSubmitSelection(singleTicket) {
    const count = selectedCount()
    if (submitInFlight || root.busy || count === 0) return false
    if (singleTicket && count <= 1) return false
    return !singleTicket || bulkName() !== ""
  }

  function canEditBulkTicket() {
    return !submitInFlight && !root.busy && selectedCount() > 1
  }

  function bulkName() {
    return String(page.bulkName || "").trim()
  }

  function resetSelection() {
    const model = quantityRows()
    for (let index = 0; index < model.count; ++index) {
      model.setProperty(index, "quantity", 0)
    }
    page.bulkName = ""
    page.selectionRevision += 1
  }

  function busyText(countOnly, singleTicket, directPrint) {
    if (directPrint) return root.appState.development_mode
    ? LocaleText.tr("Rendering ticket")
    : LocaleText.tr("Printing ticket")
    if (countOnly) return LocaleText.tr("Recording tickets")
    if (singleTicket) return root.appState.development_mode
    ? LocaleText.tr("Rendering bulk ticket")
    : LocaleText.tr("Printing bulk ticket")
    return root.appState.development_mode
    ? LocaleText.tr("Rendering tickets")
    : LocaleText.tr("Printing tickets")
  }

  function successText(countOnly, singleTicket, directPrint) {
    if (directPrint) return root.appState.development_mode
    ? LocaleText.tr("Ticket rendered")
    : LocaleText.tr("Ticket submitted to CUPS")
    if (countOnly) return LocaleText.tr("Tickets recorded")
    if (singleTicket) return root.appState.development_mode
    ? LocaleText.tr("Bulk ticket rendered")
    : LocaleText.tr("Bulk ticket submitted")
    return root.appState.development_mode
    ? LocaleText.tr("Ticket output completed")
    : LocaleText.tr("Ticket jobs submitted")
  }

  function submitSelection(countOnly, singleTicket) {
    if (submitInFlight || root.busy) return
    const requests = selectedRequests()
    const count = selectedCount()
    if (count === 0) {
      root.errorDialogObject.text = LocaleText.tr("Select at least one ticket.")
      root.errorDialogObject.open()
      return
    }
    if (singleTicket && count <= 1) {
      root.errorDialogObject.text = LocaleText.tr("Select at least two tickets for a bulk ticket.")
      root.errorDialogObject.open()
      return
    }
    if (singleTicket && bulkName() === "") {
      root.errorDialogObject.text = LocaleText.tr("Bulk ticket name is required.")
      root.errorDialogObject.open()
      return
    }
    const directPrint = !countOnly && !singleTicket && count === 1
    const busy = busyText(countOnly, singleTicket, directPrint)
    const success = successText(countOnly, singleTicket, directPrint)
    submitInFlight = true
    const started = root.runBackendBusy(busy, directPrint ? "print_one" : "bulk_print", function() {
        if (directPrint) {
          backend.printOneAsync(page.ticketId, requests[0].subticket_id, page.localDate)
        } else {
          backend.bulkPrintAsync(
            page.ticketId,
            page.localDate,
            JSON.stringify(requests),
            bulkName(),
            countOnly,
            singleTicket
          )
        }
      }, function(responseJson) {
        submitInFlight = false
        const data = root.acceptResponse(responseJson, success)
        if (data) {
          resetSelection()
          load()
        }
    })
    if (!started) submitInFlight = false
  }

  function printSelected() {
    submitSelection(false, false)
  }

  function load() {
    const data = root.acceptResponse(backend.loadTicket(page.ticketId, page.localDate))
    if (!data) return false
    page.summary = data
    syncQuantityModel()
    days.cleanupSelectedDays()
    return true
  }
}
