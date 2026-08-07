import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: counterDialog

  property var backend: root ? root.backendObject : null
  property string ticketId: ""
  property string subticketId: ""
  property string subticketLabel: ""
  property string localDate: ""
  property int currentNext: 1

  title: LocaleText.tr("Adjust daily counter")
  dialogWidth: 430

  onOpened: {
    counterValue.realValue = currentNext
    counterReason.text = LocaleText.tr("Manual correction")
  }

  function submit() {
    const data = root.acceptResponse(backend.adjustCounter(
        ticketId,
        subticketId,
        localDate,
        counterValue.realValue,
        counterReason.text
      ), LocaleText.tr("Counter adjusted"))
    if (data && root.pageStack.currentItem && root.pageStack.currentItem.load) {
      root.pageStack.currentItem.load()
    }
    counterDialog.close()
  }

  contentItem: ColumnLayout {
    spacing: counterDialog.contentSpacing

    Controls.Label {
      text: LocaleText.tr("Price: ") + counterDialog.subticketLabel
    }
    Controls.Label { text: LocaleText.tr("Current next number: ") + counterDialog.currentNext }
    Controls.Label { text: LocaleText.tr("Set next number") }
    NumberInput {
      id: counterValue
      decimals: 0
      realFrom: 1
      realTo: 999999999
      realStepSize: 1
      realValue: 1
      Layout.fillWidth: true
    }
    Controls.Label { text: LocaleText.tr("Reason") }
    Controls.TextField {
      id: counterReason
      Layout.fillWidth: true
    }
    Kirigami.InlineMessage {
      Layout.fillWidth: true
      type: Kirigami.MessageType.Warning
      text: LocaleText.tr("This does not delete or rewrite prior print records. The correction is saved as an audit entry.")
    }

    DialogActions {
      Controls.Button {
        text: LocaleText.tr("OK")
        icon.name: AppIcons.pick("ok")
        highlighted: true
        onClicked: counterDialog.submit()
      }
    }
  }
}
