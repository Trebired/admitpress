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

  property QtObject activity: PrintActivity {
    root: logic.root
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

  function priceCurrency(currency) {
    return root.appState.main_currency || currency || ""
  }

  function saleCurrencyLabelFor(amountMinor, currency) {
    const priced = priceCurrency(currency)
    const saleCurrency = page.saleCurrency || ""
    const base = MoneyFormat.format(amountMinor, priced)
    if (!saleCurrency || saleCurrency === priced) return base
    const response = root.parseResponse(
      backend.cachedConvertAmount(amountMinor, priced, saleCurrency)
    )
    const converted = response.ok ? response.data.amount_minor : null
    if (converted === null || converted === undefined) {
      return base + "  →  " + LocaleText.tr("rate loading...")
    }
    return base + "  →  " + MoneyFormat.format(converted, saleCurrency)
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
      const currency = priceCurrency(subticket.currency || "")
      model.append({
          subticket_id: subticket.id,
          amount_minor: subticket.amount_minor,
          currency: currency,
          label: saleCurrencyLabelFor(subticket.amount_minor, currency),
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
    if (page.isCurrentDayLocked && page.isCurrentDayLocked()) return
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
    const moneyText = saleCurrencyLabelFor(totalMinor(), selectedCurrency())
    return TextFormat.ticketCount(count) + ", " + moneyText
  }

  function selectionSummaryText() {
    page.selectionRevision
    return selectedCount() === 0 ? LocaleText.tr("No tickets selected") : selectionText()
  }

  function canSubmitSelection(singleTicket) {
    if (page.isCurrentDayLocked && page.isCurrentDayLocked()) return false
    const count = selectedCount()
    if (submitInFlight || root.busy || count === 0) return false
    return !singleTicket || count > 1
  }

  function canNameSale() {
    if (page.isCurrentDayLocked && page.isCurrentDayLocked()) return false
    return !submitInFlight && !root.busy && selectedCount() > 0
  }

  function saleName() {
    return String(page.saleName || "").trim()
  }

  function resetSelection() {
    const model = quantityRows()
    for (let index = 0; index < model.count; ++index) {
      model.setProperty(index, "quantity", 0)
    }
    page.saleName = ""
    page.selectionRevision += 1
  }

  function busyText(countOnly, singleTicket, directPrint) {
    if (directPrint) return root.appState.development_mode
    ? LocaleText.tr("Rendering ticket")
    : LocaleText.tr("Printing ticket")
    if (countOnly) return LocaleText.tr("Recording tickets")
    if (singleTicket) return root.appState.development_mode
    ? LocaleText.tr("Rendering group ticket")
    : LocaleText.tr("Printing group ticket")
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
    ? LocaleText.tr("Group ticket rendered")
    : LocaleText.tr("Group ticket submitted")
    return root.appState.development_mode
    ? LocaleText.tr("Ticket output completed")
    : LocaleText.tr("Ticket jobs submitted")
  }

  function showSubmitError(message) {
    root.errorDialogObject.text = message
    root.errorDialogObject.open()
  }

  function showSubmitWarning(message) {
    root.showPassiveNotification(message, "short")
  }

  function submissionBlocked(countOnly, singleTicket, count) {
    if (page.isCurrentDayLocked && page.isCurrentDayLocked()) {
      showSubmitError(LocaleText.tr("This day is locked because it already has sales and is not today."))
      return true
    }
    if (count === 0) {
      showSubmitError(LocaleText.tr("Select at least one ticket."))
      return true
    }
    if (singleTicket && count <= 1) {
      showSubmitError(LocaleText.tr("Select at least two tickets for a group ticket."))
      return true
    }
    if (!countOnly && page.hasPrintableContent && !page.hasPrintableContent(singleTicket)) {
      showSubmitWarning(LocaleText.tr("Add content in the editor before printing this ticket."))
      return true
    }
    return false
  }

  function submitSelection(countOnly, singleTicket) {
    if (submitInFlight || root.busy) return
    const requests = selectedRequests()
    const count = selectedCount()
    if (submissionBlocked(countOnly, singleTicket, count)) return
    const directPrint = !countOnly && !singleTicket && count === 1
    const busy = busyText(countOnly, singleTicket, directPrint)
    const success = successText(countOnly, singleTicket, directPrint)
    submitInFlight = true
    const saleCurrency = page.saleCurrency || ""
    const started = root.runBackendBusy(busy, directPrint ? "print_one" : "bulk_print", function() {
        if (directPrint) {
          backend.printOneAsync(
            page.ticketId,
            requests[0].subticket_id,
            page.localDate,
            saleCurrency,
            saleName()
          )
        } else {
          backend.bulkPrintAsync(
            page.ticketId,
            page.localDate,
            JSON.stringify(requests),
            saleName(),
            countOnly,
            singleTicket,
            saleCurrency
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
    if (page.refreshDayState) page.refreshDayState()
    syncQuantityModel()
    days.cleanupSelectedDays()
    return true
  }
}
