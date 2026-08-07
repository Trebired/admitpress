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
  property string dateText: localDate
  property string bulkName: ""
  property int selectionRevision: 0
  property var selectedDayMap: ({})
  property int selectedDayRevision: 0
  property int activeQuantityIndex: 0
  property string revertSubticketId: ""

  KirigamiLayouts.ColumnView.minimumWidth: Kirigami.Units.gridUnit * 42
  KirigamiLayouts.ColumnView.preferredWidth: Kirigami.Units.gridUnit * 58
  KirigamiLayouts.ColumnView.fillWidth: true
  KirigamiLayouts.ColumnView.interactiveResizeEnabled: true

  title: summary ? "Print " + summary.ticket.name : "Printing"

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

  function revertedNextNumber(row) {
    return Math.max(ticketNumberStart(), Number(row.current_next || 1) - ticketNumberIncrement())
  }

  function canRevertLastCounter() {
    selectionRevision
    const row = activeQuantityRow()
    return !!row && Number(row.current_next || 1) > ticketNumberStart()
  }

  function openRevertLastCounterDialog() {
    const row = activeQuantityRow()
    if (!row || !canRevertLastCounter()) return
    revertSubticketId = row.subticket_id
    root.revertLastDialogObject.openFor(
      printPage,
      row.label,
      Number(row.current_next || 1),
      revertedNextNumber(row)
    )
  }

  function applyRevertLastCounter(revertedNext) {
    const row = quantityRowBySubticketId(revertSubticketId) || activeQuantityRow()
    revertSubticketId = ""
    if (!row) return
    const data = root.acceptResponse(backend.adjustCounter(
        ticketId,
        row.subticket_id,
        localDate,
        Number(revertedNext),
        "Revert last count"
      ), "Last count reverted")
    if (data) load()
  }

  function selectedCount() {
    return logic.selectedCount()
  }

  function canSubmitSelection(singleTicket) {
    return logic.canSubmitSelection(singleTicket)
  }

  function selectionSummaryText() {
    return logic.selectionSummaryText()
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

  function allDaysSelected() {
    return logic.days.allDaysSelected()
  }

  function selectedDayCount() {
    return logic.days.selectedDayCount()
  }

  function selectedDaySummaryText() {
    return logic.days.selectedDaySummaryText()
  }

  function canContinueDay(localDateValue) {
    return logic.days.canContinueDay(localDateValue)
  }

  function switchToDate(localDateValue) {
    logic.days.switchToDate(localDateValue)
  }

  function continueDay(localDateValue) {
    logic.days.continueDay(localDateValue)
  }

  function shiftedDate(localDateValue, offsetDays) {
    return logic.days.shiftedDate(localDateValue, offsetDays)
  }

  function finishDay() {
    logic.days.finishDay()
  }

  function load() {
    logic.load()
  }

  function printSelected() {
    logic.printSelected()
  }

  function submitSelection(countOnly, singleTicket) {
    logic.submitSelection(countOnly, singleTicket)
  }

  function openBulkCounterDialog() {
    if (printPage.summary) root.bulkCounterDialogObject.openFor(printPage.summary, printPage.localDate)
  }

  Component.onCompleted: if (ticketId) Qt.callLater(load)

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
      text: "Development output is enabled. This screen creates files without using the printer."
    }

    ToolbarSurface {
      Layout.fillWidth: true

      Controls.Label { text: "Date"; font.bold: true }

      Controls.TextField {
        text: printPage.dateText
        placeholderText: "YYYY-MM-DD"
        inputMethodHints: Qt.ImhDate
        onEditingFinished: {
          printPage.localDate = text
          printPage.dateText = text
          printPage.load()
        }
      }

      Controls.ToolButton {
        text: "Day"
        icon.name: "view-calendar"
        onClicked: dayMenu.popup()

        Controls.Menu {
          id: dayMenu

          Controls.MenuItem {
            text: "Today"
            icon.name: "go-jump-today"
            onTriggered: printPage.switchToDate(backend.today())
          }

          Controls.MenuItem {
            text: "Finish"
            icon.name: "go-next"
            onTriggered: printPage.finishDay()
          }

          DangerMenuItem {
            text: "Reset"
            enabled: printPage.summary !== null
            onTriggered: {
              root.resetDayDialogObject.ticketId = printPage.ticketId
              root.resetDayDialogObject.localDate = printPage.localDate
              root.resetDayDialogObject.open()
            }
          }
        }
      }

      Item { Layout.fillWidth: true }

      RowLayout {
        spacing: Kirigami.Units.smallSpacing

        Kirigami.Heading {
          level: 2
          text: printPage.selectionSummaryText()
        }
        Controls.ToolButton {
          text: "Counters"
          icon.name: "overflow-menu"
          enabled: printPage.summary !== null
          onClicked: counterMenu.popup()

          Controls.Menu {
            id: counterMenu

            Controls.MenuItem {
              text: "Adjust counters"
              icon.name: "format-list-ordered"
              enabled: printPage.summary !== null
              onTriggered: printPage.openBulkCounterDialog()
            }

            DangerMenuItem {
              text: "Revert last count"
              icon.name: "edit-undo"
              enabled: printPage.canRevertLastCounter()
              onTriggered: printPage.openRevertLastCounterDialog()
            }
          }
        }
      }
    }

    PrintControlsPane {
      Layout.fillWidth: true
      Layout.fillHeight: true
      root: printPage.root
      printPage: printPage
    }
  }

  Shortcut {
    sequence: "Return"
    enabled: printPage.visible && printPage.canSubmitSelection(false)
    autoRepeat: false
    onActivated: printPage.printSelected()
  }
}
