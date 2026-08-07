import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: resetDayDialog

  property var backend: root ? root.backendObject : null
  property string ticketId: ""
  property string localDate: ""

  title: "Reset day"
  dialogWidth: 460

  function submit() {
    const data = root.acceptResponse(backend.resetDay(ticketId, localDate), "Day reset")
    if (data && root.pageStack.currentItem && root.pageStack.currentItem.load) {
      root.pageStack.currentItem.load()
    }
    resetDayDialog.close()
  }

  contentItem: ColumnLayout {

    Controls.Label {
      Layout.fillWidth: true
      text: "Reset " + resetDayDialog.localDate + " for this ticket?"
    }
    Controls.Label {
      Layout.fillWidth: true
      text: "This clears that date's print records and per-price counters. The next print for each price starts from the configured start number " +
        "again."
      wrapMode: Text.WordWrap
    }

    DialogActions {
      DangerButton {
        text: "Reset"
        onClicked: resetDayDialog.submit()
      }
    }
  }
}
