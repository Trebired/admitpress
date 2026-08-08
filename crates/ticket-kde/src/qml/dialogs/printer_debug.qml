import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

ColumnLayout {
  id: printerDebug

  property var settingsDialog
  readonly property var appRoot: settingsDialog ? settingsDialog.root : null

  Layout.fillWidth: true
  spacing: Kirigami.Units.largeSpacing

  function copyDiagnosticsText() {
    if (!diagnosticsTextArea.text || !appRoot) return
    diagnosticsTextArea.forceActiveFocus()
    diagnosticsTextArea.selectAll()
    diagnosticsTextArea.copy()
    diagnosticsTextArea.deselect()
    appRoot.showPassiveNotification(LocaleText.tr("Logs copied"))
  }

  SectionBorder {}

  RowLayout {
    Layout.fillWidth: true
    spacing: Kirigami.Units.smallSpacing

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
      icon.name: AppIcons.pick("wizard")
      text: LocaleText.tr("Repair")
      onClicked: repairMenu.popup()

      Controls.Menu {
        id: repairMenu

        Controls.MenuItem {
          icon.name: AppIcons.pick("wizard")
          text: LocaleText.tr("Fix")
          onTriggered: settingsDialog.runPrinterFix(false)
        }

        Controls.MenuItem {
          icon.name: AppIcons.pick("secure")
          text: LocaleText.tr("Repair as administrator")
          onTriggered: if (appRoot) appRoot.sudoFixDialogObject.open()
        }
      }
    }
    Controls.Button {
      icon.name: AppIcons.pick("refresh")
      text: settingsDialog.checkingPrinter ? LocaleText.tr("Checking") : LocaleText.tr("Check")
      enabled: !settingsDialog.checkingPrinter
      onClicked: settingsDialog.refreshPrinterDiagnostics()
    }
    Controls.Button {
      icon.name: AppIcons.pick("copy")
      text: LocaleText.tr("Copy logs")
      enabled: diagnosticsTextArea.text.length > 0
      onClicked: printerDebug.copyDiagnosticsText()
    }
  }

  Kirigami.InlineMessage {
    Layout.fillWidth: true
    visible: settingsDialog.printerDiagnostics !== null
    type: settingsDialog.asArray(
      settingsDialog.printerDiagnostics ? settingsDialog.printerDiagnostics.problems : []
    ).length > 0 ? Kirigami.MessageType.Warning : Kirigami.MessageType.Positive
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
