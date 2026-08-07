import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../../../components"

AppDialog {
  id: finishDayConfirmDialog

  property var backend: root ? root.backendObject : null
  property string ticketId: ""
  property string localDate: ""
  property var day: null

  title: "Finish day?"
  dialogWidth: 430

  function openFor(ticketIdValue, localDateValue, dayValue) {
    ticketId = ticketIdValue
    localDate = localDateValue
    day = dayValue
    open()
  }

  function submit() {
    finishDayConfirmDialog.close()
    root.finishDaySummaryDialogObject.openFor(ticketId, day)
  }

  contentItem: ColumnLayout {
    spacing: Kirigami.Units.largeSpacing

    Controls.Label {
      Layout.fillWidth: true
      text: "Finish " + finishDayConfirmDialog.localDate + "?"
    }
    Controls.Label {
      Layout.fillWidth: true
      text: "The date will stay on this day until you choose to continue to the next day."
      wrapMode: Text.WordWrap
    }
    RowLayout {
      Layout.fillWidth: true
      Controls.Label {
        Layout.fillWidth: true
        text: TextFormat.ticketCount(
          finishDayConfirmDialog.day ? finishDayConfirmDialog.day.ticket_count : 0
        )
      }
      Controls.Label {
        text: finishDayConfirmDialog.day
        ? root.formatMoney(finishDayConfirmDialog.day.total_minor, finishDayConfirmDialog.day.currency)
        : root.formatMoney(0, "")
      }
    }

    DialogActions {
      Controls.Button {
        text: "Finish"
        icon.name: "dialog-ok"
        highlighted: true
        onClicked: finishDayConfirmDialog.submit()
      }
    }
  }
}
