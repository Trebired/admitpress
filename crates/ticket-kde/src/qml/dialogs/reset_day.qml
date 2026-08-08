import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import "../components"

AppDialog {
  id: resetDayDialog

  property var backend: root ? root.backendObject : null
  property string ticketId: ""
  property string localDate: ""

  title: LocaleText.tr("Reset day")
  dialogWidth: 460

  function openFor(ticketIdValue, localDateValue) {
    ticketId = ticketIdValue
    localDate = localDateValue
    open()
  }

  function submit() {
    const data = root.acceptResponse(
      backend.resetDay(ticketId, localDate),
      LocaleText.tr("Day reset")
    )
    if (data && root.pageStack.currentItem && root.pageStack.currentItem.load) {
      root.pageStack.currentItem.load()
    }
    resetDayDialog.close()
  }

  contentItem: ColumnLayout {
    spacing: resetDayDialog.contentSpacing

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("Reset ")
      + TextFormat.dayDate(resetDayDialog.localDate)
      + LocaleText.tr(" for this ticket?")
    }

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("This clears that date's print records and per-price counters.")
      wrapMode: Text.WordWrap
    }

    DialogActions {
      DangerButton {
        text: LocaleText.tr("Reset day")
        icon.name: AppIcons.pick("reset")
        onClicked: resetDayDialog.submit()
      }
    }
  }
}
