import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

PrintListRow {
  id: recentPrintRow

  required property var printOperation
  signal detailRequested(var printOperation)

  readonly property string operationName: "Print"
  readonly property string operationSubtitle: {
    const label = recentPrintRow.printOperation.operation_label || ""
    const base = recentPrintRow.printOperation.status
    + "  " + recentPrintRow.printOperation.local_date
    + "  " + TextFormat.ticketCount(recentPrintRow.printOperation.ticket_count)
    return label ? base + "  " + label : base
  }

  onClicked: recentPrintRow.detailRequested(recentPrintRow.printOperation)

  Kirigami.Icon {
    Layout.preferredWidth: Kirigami.Units.iconSizes.smallMedium
    Layout.preferredHeight: Kirigami.Units.iconSizes.smallMedium
    Layout.alignment: Qt.AlignVCenter
    source: recentPrintRow.printOperation.status === "failed" ? "dialog-error" : "dialog-ok"
  }

  TicketNumberLabel {
    Layout.alignment: Qt.AlignVCenter
    value: recentPrintRow.printOperation.print_number
  }

  ColumnLayout {
    Layout.fillWidth: true
    Layout.alignment: Qt.AlignVCenter
    spacing: 0

    Controls.Label {
      Layout.fillWidth: true
      text: recentPrintRow.operationName
      elide: Text.ElideRight
    }

    Controls.Label {
      Layout.fillWidth: true
      text: recentPrintRow.operationSubtitle
      elide: Text.ElideRight
      font.pointSize: Kirigami.Theme.smallFont.pointSize
      color: Kirigami.Theme.disabledTextColor
    }
  }
}
