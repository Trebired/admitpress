import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../../../components"

AppDialog {
  id: finishDaySummaryDialog

  property var backend: root ? root.backendObject : null
  property string ticketId: ""
  property var day: null

  title: day ? "Finished " + day.local_date : "Day finished"
  dialogWidth: 540
  dialogHeight: 560

  function openFor(ticketIdValue, dayValue) {
    ticketId = ticketIdValue
    day = dayValue
    open()
  }

  function subtickets() {
    return day && day.subtickets ? day.subtickets : []
  }

  function nextDate() {
    const page = root.pageStack.currentItem
    return page && page.shiftedDate && day ? page.shiftedDate(day.local_date, 1) : backend.today()
  }

  contentItem: ColumnLayout {
    spacing: Kirigami.Units.largeSpacing

    RowLayout {
      Layout.fillWidth: true
      Controls.Label {
        Layout.fillWidth: true
        text: TextFormat.ticketCount(
          finishDaySummaryDialog.day ? finishDaySummaryDialog.day.ticket_count : 0
        )
      }
      Controls.Label {
        text: finishDaySummaryDialog.day
        ? root.formatMoney(finishDaySummaryDialog.day.total_minor, finishDaySummaryDialog.day.currency)
        : root.formatMoney(0, "")
      }
    }

    Kirigami.Separator { Layout.fillWidth: true }

    Controls.ScrollView {
      id: finishDaySummaryScroll
      Layout.fillWidth: true
      Layout.fillHeight: true
      Layout.preferredHeight: finishDaySummaryContent.implicitHeight
      clip: true
      contentWidth: availableWidth

      ColumnLayout {
        id: finishDaySummaryContent
        width: finishDaySummaryDialog.availableWidth
        spacing: Kirigami.Units.smallSpacing

        DaySalesTable {
          appRoot: root
          rows: finishDaySummaryDialog.subtickets()
          visible: rows.length > 0
        }

        Kirigami.PlaceholderMessage {
          Layout.fillWidth: true
          visible: finishDaySummaryDialog.subtickets().length === 0
          text: "No sales"
        }
      }
    }

    DialogActions {
      Controls.Button {
        text: "Continue next day"
        icon.name: "go-next"
        highlighted: true
        enabled: finishDaySummaryDialog.day !== null
        onClicked: {
          const page = root.pageStack.currentItem
          const next = finishDaySummaryDialog.nextDate()
          finishDaySummaryDialog.close()
          if (page && page.switchToDate) {
            page.switchToDate(next)
          }
        }
      }
    }
  }
}
