import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../../components"

AppDialog {
  id: deleteDayDialog

  property var backend: root ? root.backendObject : null
  property string ticketId: ""
  property string localDate: ""

  title: LocaleText.tr("Delete day")
  dialogWidth: 460

  function openFor(ticketIdValue, localDateValue) {
    ticketId = ticketIdValue
    localDate = localDateValue
    open()
  }

  function submit() {
    const data = root.acceptResponse(
      backend.deleteDay(ticketId, localDate),
      LocaleText.tr("Day deleted")
    )
    if (data && root.pageStack.currentItem && root.pageStack.currentItem.load) {
      root.pageStack.currentItem.load()
    }
    deleteDayDialog.close()
  }

  contentItem: ColumnLayout {
    spacing: deleteDayDialog.contentSpacing

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("Delete ")
      + TextFormat.dayDate(deleteDayDialog.localDate)
      + LocaleText.tr(" for this ticket?")
    }

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("This removes that date's print records and per-price counters.")
      wrapMode: Text.WordWrap
    }

    DialogActions {
      DangerButton {
        text: LocaleText.tr("Delete")
        onClicked: deleteDayDialog.submit()
      }
    }
  }
}
