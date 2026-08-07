import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

PrintListRow {
  id: daySummaryRow

  required property var day
  property var appRoot
  property var printPage
  readonly property string localDate: day ? day.local_date : ""
  readonly property string ticketCountText: TextFormat.ticketCount(day ? day.ticket_count : 0)
  readonly property string moneyText: appRoot && day ? appRoot.formatMoney(day.total_minor, day.currency) : ""

  signal detailRequested

  focusPolicy: Qt.StrongFocus
  Accessible.name: localDate + ", " + ticketCountText + ", " + moneyText
  Accessible.onPressAction: detailRequested()
  onClicked: detailRequested()

  Controls.CheckBox {
    Layout.preferredWidth: Kirigami.Units.gridUnit * 1.5
    Layout.alignment: Qt.AlignVCenter
    checked: daySummaryRow.printPage
    ? daySummaryRow.printPage.daySelected(daySummaryRow.localDate)
    : false
    onClicked: {
      if (daySummaryRow.printPage) {
        daySummaryRow.printPage.setDaySelected(daySummaryRow.localDate, checked)
      }
    }
  }

  Controls.Label {
    Layout.fillWidth: true
    Layout.minimumWidth: Kirigami.Units.gridUnit * 5
    Layout.alignment: Qt.AlignVCenter
    text: daySummaryRow.localDate
    elide: Text.ElideRight
  }

  Controls.Label {
    Layout.preferredWidth: Kirigami.Units.gridUnit * 4.8
    Layout.alignment: Qt.AlignVCenter
    text: daySummaryRow.ticketCountText
    horizontalAlignment: Text.AlignRight
    elide: Text.ElideRight
  }

  Controls.Label {
    Layout.preferredWidth: Kirigami.Units.gridUnit * 5
    Layout.alignment: Qt.AlignVCenter
    text: daySummaryRow.moneyText
    horizontalAlignment: Text.AlignRight
    elide: Text.ElideRight
  }
}
