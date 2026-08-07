import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../../components"

AppDialog {
  id: printOperationDialog

  property var operation: null
  property var ticket: null

  title: operation ? LocaleText.tr("Print #") + operation.print_number : LocaleText.tr("Print details")
  dialogWidth: 560
  dialogHeight: 620

  function openOperation(operationValue, ticketValue) {
    operation = operationValue || null
    ticket = ticketValue || null
    if (!operation) return
    open()
  }

  function operationKind() {
    if (!operation) return ""
    return LocaleText.tr("Print")
  }

  function moneyText(amountMinor, currency) {
    return root ? root.formatMoney(amountMinor, currency || "") : ""
  }

  function ticketPadding() {
    return ticket ? Number(ticket.number_padding || 0) : 0
  }

  function ticketNumbersText(item) {
    const numbers = item && item.ticket_numbers ? item.ticket_numbers : []
    if (numbers.length === 0) return LocaleText.tr("None")
    return numbers.map(number => TextFormat.displayTicketNumber(number, ticketPadding())).join(", ")
  }

  function statusText() {
    if (!operation) return ""
    if (operation.status === "queued") return LocaleText.tr("Queued")
    if (operation.status === "completed") return LocaleText.tr("Completed")
    if (operation.status === "failed") return LocaleText.tr("Failed")
    return operation.status
  }

  function successText() {
    if (!operation) return ""
    return operation.status === "completed" || operation.status === "queued"
    ? LocaleText.tr("Yes")
    : LocaleText.tr("No")
  }

  contentItem: Controls.ScrollView {
    id: operationScroll
    clip: true
    contentWidth: availableWidth
    implicitHeight: operationContent.implicitHeight

    ColumnLayout {
      id: operationContent
      width: operationScroll.availableWidth
      spacing: Kirigami.Units.largeSpacing

      RowLayout {
        Layout.fillWidth: true
        Kirigami.Icon {
          Layout.preferredWidth: Kirigami.Units.iconSizes.medium
          Layout.preferredHeight: Kirigami.Units.iconSizes.medium
          source: operation && operation.status === "failed" ? "dialog-error" : "dialog-ok"
        }
        ColumnLayout {
          Layout.fillWidth: true
          spacing: 0
          Controls.Label {
            Layout.fillWidth: true
            text: printOperationDialog.operationKind()
          }
          Controls.Label {
            Layout.fillWidth: true
            text: operation && operation.operation_label ? operation.operation_label : ""
            visible: text !== ""
            color: Kirigami.Theme.disabledTextColor
            elide: Text.ElideRight
          }
        }
      }

      GridLayout {
        Layout.fillWidth: true
        columns: 2
        rowSpacing: Kirigami.Units.smallSpacing
        columnSpacing: Kirigami.Units.largeSpacing

        Controls.Label { text: LocaleText.tr("Status"); color: Kirigami.Theme.disabledTextColor }
        Controls.Label { Layout.fillWidth: true; text: printOperationDialog.statusText() }
        Controls.Label { text: LocaleText.tr("Successful"); color: Kirigami.Theme.disabledTextColor }
        Controls.Label { Layout.fillWidth: true; text: printOperationDialog.successText() }
        Controls.Label { text: LocaleText.tr("Date"); color: Kirigami.Theme.disabledTextColor }
        Controls.Label { Layout.fillWidth: true; text: operation ? operation.local_date : "" }
        Controls.Label { text: LocaleText.tr("Tickets"); color: Kirigami.Theme.disabledTextColor }
        Controls.Label {
          Layout.fillWidth: true
          text: operation ? TextFormat.ticketCount(operation.ticket_count) : ""
        }
        Controls.Label { text: LocaleText.tr("Total"); color: Kirigami.Theme.disabledTextColor }
        Controls.Label {
          Layout.fillWidth: true
          text: operation ? printOperationDialog.moneyText(operation.total_minor, operation.currency) : ""
        }
      }

      Kirigami.Separator { Layout.fillWidth: true }

      DaySalesTable {
        appRoot: root
        rows: operation && operation.items ? operation.items : []
        visible: rows.length > 0
      }

      Repeater {
        model: operation && operation.items ? operation.items : []
        delegate: ColumnLayout {
          required property var modelData

          Layout.fillWidth: true
          spacing: 0

          Controls.Label {
            Layout.fillWidth: true
            text: modelData.display_label || modelData.name ||
              printOperationDialog.moneyText(modelData.amount_minor, modelData.currency)
          }
          Controls.Label {
            Layout.fillWidth: true
            text: printOperationDialog.ticketNumbersText(modelData)
            wrapMode: Text.WordWrap
            color: Kirigami.Theme.disabledTextColor
          }
        }
      }

      Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: operation && operation.status === "failed"
        type: Kirigami.MessageType.Error
        text: operation && operation.failure_reason ? operation.failure_reason : LocaleText.tr("Print failed")
      }

      GridLayout {
        Layout.fillWidth: true
        columns: 2
        rowSpacing: Kirigami.Units.smallSpacing
        columnSpacing: Kirigami.Units.largeSpacing

        Controls.Label { text: LocaleText.tr("Printer"); color: Kirigami.Theme.disabledTextColor }
        Controls.Label {
          Layout.fillWidth: true
          text: operation && operation.printer_name ? operation.printer_name : LocaleText.tr("None")
          elide: Text.ElideRight
        }
        Controls.Label { text: LocaleText.tr("Job"); color: Kirigami.Theme.disabledTextColor }
        Controls.Label {
          Layout.fillWidth: true
          text: operation && operation.printer_job_id ? operation.printer_job_id : LocaleText.tr("None")
          elide: Text.ElideRight
        }
        Controls.Label { text: LocaleText.tr("Output"); color: Kirigami.Theme.disabledTextColor }
        Controls.Label {
          Layout.fillWidth: true
          text: operation && operation.rendered_output_path ? operation.rendered_output_path : LocaleText.tr("None")
          elide: Text.ElideMiddle
        }
      }
    }
  }
}
