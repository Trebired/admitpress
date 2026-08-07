import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import "../components"

AppDialog {
  id: revertLastDialog

  property var printPage: null
  property string priceLabel: ""
  property int currentNext: 0
  property int revertedNext: 0

  title: LocaleText.tr("Revert last count")
  dialogWidth: 460

  function openFor(page, label, current, reverted) {
    printPage = page
    priceLabel = label
    currentNext = current
    revertedNext = reverted
    open()
  }

  function submit() {
    const page = printPage
    close()
    printPage = null
    if (page && page.applyRevertLastCounter) page.applyRevertLastCounter(revertLastDialog.revertedNext)
  }

  onClosed: printPage = null

  contentItem: ColumnLayout {

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("Revert the last count for ") + revertLastDialog.priceLabel + "?"
    }
    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("The next number goes back from ")
      + revertLastDialog.currentNext
      + LocaleText.tr(" to ")
      + revertLastDialog.revertedNext
      + LocaleText.tr(". Print records are kept, and the correction is stored as an audit entry.")
      wrapMode: Text.WordWrap
    }

    DialogActions {
      DangerButton {
        text: LocaleText.tr("Revert count")
        icon.name: "edit-undo"
        onClicked: revertLastDialog.submit()
      }
    }
  }
}
