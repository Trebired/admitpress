import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: dayDetailDialog

  property var backend: root ? root.backendObject : null
  property string ticketId: ""
  property var day: null

  title: day ? "Day " + day.local_date : "Day details"
  dialogWidth: 520
  dialogHeight: 520

  function openFor(ticketIdValue, dayValue) {
    assignDay(ticketIdValue, dayValue)
    open()
  }

  function assignDay(ticketIdValue, dayValue) {
    ticketId = ticketIdValue
    day = dayValue
  }

  function subtickets() {
    if (!day || !day.subtickets) return []
    return day.subtickets
  }

  contentItem: Controls.ScrollView {
    id: dayDetailScroll
    clip: true
    contentWidth: availableWidth
    implicitHeight: dayDetailContent.implicitHeight

    ColumnLayout {
      id: dayDetailContent
      width: dayDetailDialog.availableWidth
      spacing: Kirigami.Units.largeSpacing

      RowLayout {
        Layout.fillWidth: true
        Controls.Label {
          Layout.fillWidth: true
          text: dayDetailDialog.day ? TextFormat.ticketCount(dayDetailDialog.day.ticket_count) : ""
        }
        Controls.Label {
          text: dayDetailDialog.day
          ? root.formatMoney(dayDetailDialog.day.total_minor, dayDetailDialog.day.currency)
          : ""
        }
      }

      Kirigami.Separator { Layout.fillWidth: true }

      DaySalesTable {
        appRoot: root
        rows: dayDetailDialog.subtickets()
        visible: rows.length > 0
      }

      Kirigami.PlaceholderMessage {
        Layout.fillWidth: true
        visible: dayDetailDialog.subtickets().length === 0
        text: LocaleText.tr("No sales")
      }

      RowLayout {
        Layout.fillWidth: true
        spacing: Kirigami.Units.smallSpacing

        DangerButton {
          Layout.fillWidth: true
          text: LocaleText.tr("Delete day")
          enabled: dayDetailDialog.day !== null
          onClicked: {
            dayDetailDialog.close()
            root.deleteDayDialogObject.openFor(
              dayDetailDialog.ticketId,
              dayDetailDialog.day.local_date
            )
          }
        }
      }
    }
  }
}
