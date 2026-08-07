import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: settingsDialog

  property var backend: root ? root.backendObject : null
  property var printerDiagnostics: null
  property var lastPrinterFix: null
  property bool checkingPrinter: false
  property int printerDiagnosticsRequestCounter: 0
  property string activePrinterDiagnosticsRequestId: ""

  title: LocaleText.tr("Settings")
  dialogWidth: 760
  dialogHeight: 680

  function asArray(value) {
    return Array.isArray(value) ? value : []
  }

  function applyPrinterDiagnostics(diagnostics) {
    if (!diagnostics) {
      return
    }
    printerDiagnostics = diagnostics
    const state = root.appState || {}
    root.appState = {
      tickets: state.tickets || [],
      printer_queues: asArray(diagnostics.detected_queues),
      selected_printer_queue: diagnostics.selected_queue || "",
      development_mode: diagnostics.development_mode,
      data_dir: state.data_dir || "data"
    }
  }

  function clearPrinterDiagnostics() {
    printerDiagnostics = null
    lastPrinterFix = null
    checkingPrinter = false
    activePrinterDiagnosticsRequestId = ""
  }

  function refreshPrinterDiagnostics() {
    if (checkingPrinter) {
      return
    }
    printerDiagnostics = null
    lastPrinterFix = null
    checkingPrinter = true
    printerDiagnosticsRequestCounter += 1
    activePrinterDiagnosticsRequestId = String(printerDiagnosticsRequestCounter)
    try {
      backend.printerDiagnosticsAsync(activePrinterDiagnosticsRequestId)
    } catch (error) {
      checkingPrinter = false
      activePrinterDiagnosticsRequestId = ""
      root.errorDialogObject.text = LocaleText.tr("Printer diagnostics failed: ") + error
      root.errorDialogObject.open()
    }
  }

  function handlePrinterDiagnosticsResponse(responseJson, requestId) {
    if (!opened || requestId !== activePrinterDiagnosticsRequestId) {
      return
    }
    checkingPrinter = false
    activePrinterDiagnosticsRequestId = ""
    const data = root.acceptResponse(responseJson)
    if (data) {
      settingsDialog.applyPrinterDiagnostics(data)
    }
  }

  function detectedQueuesText() {
    const diagnostics = settingsDialog.printerDiagnostics
    if (!diagnostics) {
      return ""
    }
    const queues = asArray(diagnostics.detected_queues)
    return queues.length > 0
    ? queues.join("\n")
    : LocaleText.tr("No CUPS queues detected. Add the PF8t in KDE System Settings first.")
  }

  function copyDiagnosticsText() {
    if (!diagnosticsTextArea.text) {
      return
    }
    diagnosticsTextArea.forceActiveFocus()
    diagnosticsTextArea.selectAll()
    diagnosticsTextArea.copy()
    diagnosticsTextArea.deselect()
    root.showPassiveNotification(LocaleText.tr("Logs copied"))
  }

  function runPrinterFix(useAdmin) {
    root.runBackendBusy(useAdmin ? LocaleText.tr("Fixing printer with sudo") : LocaleText.tr("Fixing printer"), "fix_printer", function() {
        backend.fixPrinterAsync(useAdmin)
      }, function(responseJson) {
        if (!settingsDialog.opened) {
          return
        }
        const data = root.acceptResponse(
          responseJson,
          useAdmin ? LocaleText.tr("Admin printer fix attempted") : LocaleText.tr("Printer fix attempted")
        )
        if (data) {
          settingsDialog.lastPrinterFix = data
          settingsDialog.applyPrinterDiagnostics(data.diagnostics_after)
        }
    })
  }

  function problemText() {
    const diagnostics = settingsDialog.printerDiagnostics
    if (!diagnostics) {
      return LocaleText.tr("Printer diagnostics have not been loaded yet.")
    }
    const problems = asArray(diagnostics.problems)
    return problems.length > 0 ? problems.join("\n") : LocaleText.tr("No printer problems reported by CUPS.")
  }

  function appendCommandReport(lines, report) {
    lines.push("")
    lines.push("$ " + report.command)
    if (report.timed_out) {
      lines.push("[timed out]")
    }
    if (report.exit_code !== undefined && report.exit_code !== null) {
      lines.push("[exit " + report.exit_code + "]")
    }
    if (report.stdout) {
      lines.push(report.stdout)
    }
    if (report.stderr) {
      lines.push("stderr:")
      lines.push(report.stderr)
    }
  }

  function diagnosticsText() {
    const diagnostics = settingsDialog.printerDiagnostics
    if (!diagnostics) {
      return ""
    }
    const lines = []
    const queues = asArray(diagnostics.detected_queues)
    lines.push("Selected queue: " + (diagnostics.selected_queue || "none"))
    lines.push("Default queue: " + (diagnostics.default_queue || "none"))
    lines.push("Development mode: " + (diagnostics.development_mode ? "on" : "off"))
    lines.push("Detected queues: " + (queues.length > 0 ? queues.join(", ") : "none"))
    const suggested = asArray(diagnostics.suggested_commands)
    if (suggested.length > 0) {
      lines.push("")
      lines.push("Suggested commands:")
      for (let command of suggested) {
        lines.push(command)
      }
    }
    const failures = asArray(diagnostics.recent_failures)
    if (failures.length > 0) {
      lines.push("")
      lines.push("Recent app print failures:")
      for (let failure of failures) {
        lines.push(TextFormat.displayTicketNumber(failure.allocated_number) + " " + failure.status + " "
          + failure.local_date + " " + (failure.failure_reason || ""))
      }
    }
    const fix = settingsDialog.lastPrinterFix
    if (fix) {
      lines.push("")
      lines.push("Last fix attempt:")
      lines.push("Mode: " + (fix.used_admin ? "sudo/admin" : "without sudo"))
      lines.push("Selected queue: " + (fix.selected_queue || "none"))
      lines.push("Changed app queue: " + (fix.changed_selected_queue ? "yes" : "no"))
      const fixCommands = asArray(fix.commands)
      for (let report of fixCommands) {
        appendCommandReport(lines, report)
      }
    }
    const commands = asArray(diagnostics.commands)
    if (commands.length > 0) {
      lines.push("")
      lines.push("Command output:")
      for (let report of commands) {
        appendCommandReport(lines, report)
      }
    }
    return lines.join("\n")
  }

  onOpened: refreshPrinterDiagnostics()
  onClosed: clearPrinterDiagnostics()

  contentItem: Controls.ScrollView {
    id: settingsScroll
    clip: true
    contentWidth: availableWidth
    implicitHeight: settingsContent.implicitHeight

    ColumnLayout {
      id: settingsContent
      width: settingsScroll.availableWidth
      spacing: Kirigami.Units.largeSpacing

      Kirigami.InlineMessage {
        Layout.fillWidth: true
        type: root.appState.development_mode ? Kirigami.MessageType.Information : Kirigami.MessageType.Warning
        text: root.appState.development_mode
        ? LocaleText.tr("Print actions currently create SVG files only.")
        : LocaleText.tr("Physical printing is enabled. Print actions are sent to the selected CUPS queue.")
      }

      RowLayout {
        Layout.fillWidth: true
        Controls.Label {
          Layout.fillWidth: true
          text: LocaleText.tr("Printer queue")
        }
      }
      RowLayout {
        Layout.fillWidth: true
        Controls.ComboBox {
          id: printerQueueCombo
          Layout.fillWidth: true
          model: root.printerChoices()
          currentIndex: root.printerChoiceIndex()
          enabled: !settingsDialog.checkingPrinter
          onActivated: function(index) {
            const queue = index <= 0 ? "" : printerQueueCombo.textAt(index)
            root.acceptResponse(backend.setPrinterQueue(queue), LocaleText.tr("Printer queue updated"))
            settingsDialog.refreshPrinterDiagnostics()
          }
        }
        Controls.Button {
          icon.name: "view-refresh"
          text: LocaleText.tr("Refresh")
          enabled: !settingsDialog.checkingPrinter
          onClicked: {
            root.reloadState()
            settingsDialog.refreshPrinterDiagnostics()
          }
        }
      }

      Controls.Switch {
        Layout.fillWidth: true
        text: LocaleText.tr("Development output, do not use paper")
        checked: root.appState.development_mode
        onClicked: {
          root.acceptResponse(backend.setDevelopmentMode(checked), LocaleText.tr("Printing mode updated"))
          settingsDialog.refreshPrinterDiagnostics()
        }
      }

      Controls.Label { text: LocaleText.tr("Data directory") }
      RowLayout {
        Layout.fillWidth: true
        Controls.TextField {
          Layout.fillWidth: true
          readOnly: true
          text: root.appState.data_dir
        }
        Controls.Button {
          text: LocaleText.tr("Choose...")
          icon.name: "folder-open"
          onClicked: root.dataDirectoryDialogObject.open()
        }
      }
      Controls.Label {
        Layout.fillWidth: true
        text: LocaleText.tr("Changing the directory opens or creates a separate ")
        + root.appDisplayName
        + LocaleText.tr(" database there.")
        wrapMode: Text.WordWrap
        color: Kirigami.Theme.disabledTextColor
      }

      Kirigami.Separator { Layout.fillWidth: true }

      Controls.Label {
        visible: settingsDialog.printerDiagnostics !== null
        text: LocaleText.tr("Detected CUPS queues")
      }
      Controls.Label {
        Layout.fillWidth: true
        visible: settingsDialog.printerDiagnostics !== null
        text: settingsDialog.detectedQueuesText()
        wrapMode: Text.WordWrap
      }

      Kirigami.Separator { Layout.fillWidth: true }

      RowLayout {
        Layout.fillWidth: true
        Controls.Label {
          Layout.fillWidth: true
          text: LocaleText.tr("Printer debug")
        }
        Controls.BusyIndicator {
          visible: settingsDialog.checkingPrinter
          running: visible
          Layout.preferredWidth: Kirigami.Units.iconSizes.smallMedium
          Layout.preferredHeight: Kirigami.Units.iconSizes.smallMedium
        }
        Controls.Button {
          icon.name: "view-refresh"
          text: settingsDialog.checkingPrinter ? LocaleText.tr("Checking") : LocaleText.tr("Check")
          enabled: !settingsDialog.checkingPrinter
          onClicked: settingsDialog.refreshPrinterDiagnostics()
        }
        Controls.Button {
          icon.name: "edit-copy"
          text: LocaleText.tr("Copy logs")
          enabled: diagnosticsTextArea.text.length > 0
          onClicked: settingsDialog.copyDiagnosticsText()
        }
      }

      RowLayout {
        Layout.fillWidth: true
        Controls.Button {
          Layout.fillWidth: true
          text: LocaleText.tr("Fix without sudo")
          icon.name: "tools-wizard"
          onClicked: settingsDialog.runPrinterFix(false)
        }
        Controls.Button {
          Layout.fillWidth: true
          text: LocaleText.tr("Fix with sudo")
          icon.name: "security-high"
          highlighted: true
          onClicked: root.sudoFixDialogObject.open()
        }
      }

      Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: settingsDialog.printerDiagnostics !== null
        type: settingsDialog.asArray(settingsDialog.printerDiagnostics ? settingsDialog.printerDiagnostics.problems : []).length > 0
        ? Kirigami.MessageType.Warning
        : Kirigami.MessageType.Positive
        text: settingsDialog.problemText()
      }

      Controls.TextArea {
        id: diagnosticsTextArea
        Layout.fillWidth: true
        Layout.preferredHeight: 280
        readOnly: true
        selectByMouse: true
        wrapMode: Text.WrapAnywhere
        font.family: "monospace"
        text: settingsDialog.diagnosticsText()
      }
    }
  }
}
