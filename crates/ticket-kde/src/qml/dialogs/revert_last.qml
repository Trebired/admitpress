import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import "../components"

AppDialog {
  id: revertLastDialog

  property var printPage: null
  property var operation: null

  title: LocaleText.tr("Revert last sale")
  dialogWidth: 460

  function openFor(page, operationValue) {
    printPage = page
    operation = operationValue || null
    open()
  }

  function saleSummary() {
    if (!operation) return ""
    const parts = [TextFormat.ticketCount(operation.ticket_count)]
    if (operation.operation_label) parts.push(operation.operation_label)
    return parts.join("  ")
  }

  function submit() {
    const page = printPage
    close()
    printPage = null
    if (page && page.applyRevertLastSale) page.applyRevertLastSale()
  }

  onClosed: {
    printPage = null
    operation = null
  }

  contentItem: ColumnLayout {
    spacing: revertLastDialog.contentSpacing

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("Revert the last sale?")
    }

    Controls.Label {
      Layout.fillWidth: true
      text: revertLastDialog.saleSummary()
      wrapMode: Text.WordWrap
    }

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("Every counter this sale advanced goes back by the number of "
        + "tickets it sold. Print records are kept, and each correction is stored as an audit entry.")
      wrapMode: Text.WordWrap
    }

    DialogActions {
      DangerButton {
        text: LocaleText.tr("Revert sale")
        icon.name: AppIcons.pick("undo")
        onClicked: revertLastDialog.submit()
      }
    }
  }
}
