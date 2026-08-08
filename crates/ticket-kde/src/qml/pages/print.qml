import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.kde.kirigami.layouts as KirigamiLayouts
import "../components"
import "../print"

Kirigami.Page {
  id: printPage

  required property var root
  required property var backend
  property string ticketId: ""
  property var summary: null
  property string localDate: backend.today()
  property string saleName: ""
  property int selectionRevision: 0
  property var selectedDayMap: ({})
  property int selectedDayRevision: 0
  property bool showEmptyDays: true
  property var collapsedMonths: ({})
  property var lockedDateMap: ({})
  property int dayStateRevision: 0
  property int activeQuantityIndex: 0
  property int activePrintTabIndex: 0
  property string saleCurrency: ""
  property string observedMainCurrency: root.appState.main_currency || ""
  property string lastObservedMainCurrency: observedMainCurrency
  property string activityFilter: ""
  property string activitySearch: ""

  KirigamiLayouts.ColumnView.fillWidth: true
  Component.onCompleted: PageColumn.configure(KirigamiLayouts.ColumnView, {
      minimumWidth: Kirigami.Units.gridUnit * 42,
      preferredWidth: Kirigami.Units.gridUnit * 58,
      interactiveResizeEnabled: true
  })

  title: summary ? LocaleText.tr("Print ") + summary.ticket.name : LocaleText.tr("Printing")
  onObservedMainCurrencyChanged: {
    handleMainCurrencyChanged(lastObservedMainCurrency, observedMainCurrency)
    lastObservedMainCurrency = observedMainCurrency
  }

  PrintLogic {
    id: logic
    root: printPage.root
    backend: printPage.backend
    page: printPage
  }

  function ticketNumberPadding() {
    return logic.ticketNumberPadding()
  }

  function displayTicketNumber(value) {
    return logic.displayTicketNumber(value)
  }

  function setSelectionQuantity(index, quantity) {
    logic.setSelectionQuantity(index, quantity)
  }

  function setActiveQuantityIndex(index) {
    if (index >= 0 && index < quantityModel.count) activeQuantityIndex = index
  }

  function quantityRowAt(index) {
    return index >= 0 && index < quantityModel.count ? quantityModel.get(index) : null
  }

  function activeQuantityRow() {
    return quantityRowAt(activeQuantityIndex) || quantityRowAt(0)
  }

  function quantityRowBySubticketId(subticketId) {
    for (let index = 0; index < quantityModel.count; ++index) {
      const row = quantityModel.get(index)
      if (row.subticket_id === subticketId) return row
    }
    return null
  }

  function ticketNumberStart() {
    return summary && summary.ticket ? Number(summary.ticket.number_start || 1) : 1
  }

  function ticketNumberIncrement() {
    return summary && summary.ticket ? Math.max(1, Number(summary.ticket.number_increment || 1)) : 1
  }

  function lastSaleOperation() {
    const operations = summary && summary.date_print_operations
    ? summary.date_print_operations
    : []
    return operations.length > 0 ? operations[0] : null
  }

  function canRevertLastSale() {
    if (isCurrentDayLocked()) return false
    return lastSaleOperation() !== null
  }

  function openRevertLastSaleDialog() {
    const operation = lastSaleOperation()
    if (!operation || !canRevertLastSale()) return
    root.revertLastDialogObject.openFor(printPage, operation)
  }

  function applyRevertLastSale() {
    const data = root.acceptResponse(
      backend.revertLastSale(ticketId, localDate),
      LocaleText.tr("Last sale reverted")
    )
    if (data) load()
  }

  function selectedCount() {
    return logic.selectedCount()
  }

  function canSubmitSelection(singleTicket) {
    return logic.canSubmitSelection(singleTicket)
  }

  function fieldHasPrintableContent(field) {
    if (!field) return false
    const text = String(field.text || "").trim()
    if (field.field_kind === "image") return text.length > 0
    if (field.field_kind === "dynamic") return !!field.dynamic_kind || text.length > 0
    return text.length > 0
  }

  function hasPrintableContent(singleTicket) {
    const ticket = summary && summary.ticket
    if (!ticket) return false
    if (String(ticket.background_svg_path || "").trim().length > 0) return true
    const templateKind = singleTicket ? "bulk" : "classic"
    const fields = summary.fields || []
    for (let index = 0; index < fields.length; ++index) {
      const field = fields[index]
      const fieldTemplateKind = field && field.template_kind === "bulk" ? "bulk" : "classic"
      if (fieldTemplateKind === templateKind && fieldHasPrintableContent(field)) return true
    }
    return false
  }

  function canNameSale() {
    return logic.canNameSale()
  }

  function selectionSummaryText() {
    return logic.selectionSummaryText()
  }

  function activityEntries() {
    return logic.activity.entries()
  }

  function activityFilterChoices() {
    return logic.activity.filterChoices()
  }

  function hasActivity() {
    return logic.activity.hasEntries()
  }

  function refreshSaleCurrencyDisplay() {
    logic.syncQuantityModel()
  }

  function handleMainCurrencyChanged(previousCurrency, currentCurrency) {
    if (saleCurrency === previousCurrency || saleCurrency === currentCurrency) {
      saleCurrency = ""
    }
    logic.syncQuantityModel()
  }

  function setSaleCurrency(value) {
    saleCurrency = value || ""
    const mainCurrency = root.appState.main_currency || ""
    if (saleCurrency && saleCurrency !== mainCurrency) {
      backend.refreshExchangeRateAsync(mainCurrency, saleCurrency)
    }
    logic.syncQuantityModel()
  }

  function selectedRequests() {
    return logic.selectedRequests()
  }

  function dayBreakdownText(day) {
    return logic.days.dayBreakdownText(day)
  }

  function daySummaries() {
    return logic.days.daySummaries()
  }

  function daySelected(localDateValue) {
    return logic.days.daySelected(localDateValue)
  }

  function setDaySelected(localDateValue, selected) {
    logic.days.setDaySelected(localDateValue, selected)
  }

  function selectAllDays() {
    logic.days.selectAllDays()
  }

  function clearSelectedDays() {
    logic.days.clearSelectedDays()
  }

  function selectedDayCount() {
    return logic.days.selectedDayCount()
  }

  function selectedDaySalesRows() {
    return logic.days.selectedDaySalesRows()
  }

  function visibleDaySummaries() {
    return logic.days.visibleDaySummaries()
  }

  function visibleMonthGroups() {
    return logic.days.visibleMonthGroups()
  }

  function monthLabel(monthKeyValue) {
    return logic.days.monthLabel(monthKeyValue)
  }

  function monthSelectionState(monthKeyValue) {
    return logic.days.monthSelectionState(monthKeyValue)
  }

  function setMonthSelected(monthKeyValue, selected) {
    logic.days.setMonthSelected(monthKeyValue, selected)
  }

  function allSelectionState() {
    return logic.days.allSelectionState()
  }

  function hasSelectableDays() {
    return logic.days.hasSelectableDays()
  }

  function openSelectedSalesInfoDialog() {
    if (selectedDayCount() === 0) return
    root.salesInfoDialogObject.openFor(
      logic.days.selectedDayTicketCount(),
      logic.days.selectedDayTotalMinor(),
      logic.days.selectedDayCurrency(),
      logic.days.selectedDaySalesRows()
    )
  }

  function currentMonthKey() {
    return String(localDate || "").substring(0, 7)
  }

  function isMonthCollapsed(monthKeyValue) {
    const explicit = collapsedMonths[monthKeyValue]
    if (explicit !== undefined) return explicit
    return monthKeyValue !== currentMonthKey()
  }

  function toggleMonthCollapsed(monthKeyValue) {
    const next = Object.assign({}, collapsedMonths)
    next[monthKeyValue] = !isMonthCollapsed(monthKeyValue)
    collapsedMonths = next
  }

  function selectedDayDates() {
    return logic.days.selectedDayDates()
  }

  function canResetSelectedDay() {
    return logic.days.canResetSelectedDay()
  }

  function openResetSelectedDayDialog() {
    if (!canResetSelectedDay()) return
    root.resetDayDialogObject.openFor(ticketId, logic.days.selectedResetDay())
  }

  function setShowEmptyDays(value) {
    showEmptyDays = !!value
    logic.days.cleanupSelectedDays()
  }

  function switchToDate(localDateValue) {
    logic.days.switchToDate(localDateValue)
  }

  function isDayLocked(localDateValue) {
    dayStateRevision
    return !!lockedDateMap[String(localDateValue || "")]
  }

  function isCurrentDayLocked() {
    return isDayLocked(localDate)
  }

  function refreshDayState() {
    lockedDateMap = logic.days.lockedDateMap()
    dayStateRevision += 1
  }

  function exportSelectedDays(format) {
    const dates = selectedDayDates()
    if (dates.length === 0 || !summary || !summary.ticket) return
    root.dayExportDialogObject.openFor(ticketId, summary.ticket.name, dates, format)
  }

  function load() {
    if (!ticketId) return false
    return logic.load()
  }

  function prepareForTicket(ticketIdValue) {
    ticketId = String(ticketIdValue || "")
    return load()
  }

  function printSelected() {
    logic.printSelected()
  }

  function resetSelection() {
    logic.resetSelection()
  }

  function submitSelection(countOnly, singleTicket) {
    logic.submitSelection(countOnly, singleTicket)
  }

  function openBulkCounterDialog() {
    if (printPage.summary && !isCurrentDayLocked()) {
      root.bulkCounterDialogObject.openFor(printPage.summary, printPage.localDate)
    }
  }

  ListModel {
    id: printQuantityModel
    dynamicRoles: true
  }

  property alias quantityModel: printQuantityModel

  ColumnLayout {
    anchors.fill: parent
    spacing: Kirigami.Units.largeSpacing

    Kirigami.InlineMessage {
      Layout.fillWidth: true
      visible: root.appState.development_mode
      type: Kirigami.MessageType.Information
      text: LocaleText.tr("Development output is enabled. This screen creates files without using the printer.")
    }

    Controls.TabBar {
      Layout.fillWidth: true
      currentIndex: printPage.activePrintTabIndex
      onCurrentIndexChanged: printPage.activePrintTabIndex = currentIndex

      Controls.TabButton {
        text: LocaleText.tr("Print")
      }

      Controls.TabButton {
        text: LocaleText.tr("Sales")
      }
    }

    StackLayout {
      Layout.fillWidth: true
      Layout.fillHeight: true
      currentIndex: printPage.activePrintTabIndex

      PrintControlsPane {
        root: printPage.root
        printPage: printPage
      }

      PrintDaysPane {
        root: printPage.root
        printPage: printPage
      }
    }
  }

  Shortcut {
    sequence: "Return"
    enabled: printPage.visible && printPage.canSubmitSelection(false)
    autoRepeat: false
    onActivated: printPage.printSelected()
  }
}
