import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../../components"

AppDialog {
  id: bulkCounterDialog

  property var backend: root ? root.backendObject : null
  property var summary: null
  property string localDate: ""
  property int revision: 0
  property int numberPadding: summary && summary.ticket ? summary.ticket.number_padding : 0

  title: LocaleText.tr("Adjust counters")
  dialogWidth: 620
  dialogHeight: 560

  ListModel { id: counterModel }

  function openFor(ticketSummary, localDateValue) {
    summary = ticketSummary
    localDate = localDateValue || backend.today()
    counterModel.clear()
    for (let subticket of ticketSummary.subtickets) {
      counterModel.append({
          subticket_id: subticket.id,
          label: root.formatMoney(subticket.amount_minor, subticket.currency),
          current_next: Number(subticket.next_number_today || 1),
          quantity: Number(subticket.next_number_today || 1)
      })
    }
    reasonField.text = LocaleText.tr("Counter correction")
    revision += 1
    open()
  }

  function changedCount() {
    let count = 0
    for (let index = 0; index < counterModel.count; ++index) {
      const item = counterModel.get(index)
      if (Number(item.quantity) !== Number(item.current_next)) count += 1
    }
    return count
  }

  function submit() {
    let changed = 0
    for (let index = 0; index < counterModel.count; ++index) {
      const item = counterModel.get(index)
      if (Number(item.quantity) === Number(item.current_next)) continue
      const data = root.acceptResponse(backend.adjustCounter(
          summary.ticket.id,
          item.subticket_id,
          localDate,
          Number(item.quantity),
          reasonField.text
      ))
      if (!data) return
      changed += 1
    }
    if (changed > 0 && root.pageStack.currentItem && root.pageStack.currentItem.load) {
      root.pageStack.currentItem.load()
      root.showPassiveNotification(LocaleText.tr("Counters adjusted"))
    }
    bulkCounterDialog.close()
  }

  contentItem: Controls.ScrollView {
    id: counterScroll
    clip: true
    contentWidth: availableWidth
    implicitHeight: counterContent.implicitHeight

    ColumnLayout {
      id: counterContent
      width: counterScroll.availableWidth
      spacing: Kirigami.Units.largeSpacing

      Controls.Label { text: LocaleText.tr("Date: ") + bulkCounterDialog.localDate }

      TicketNumberGrid {
        Layout.fillWidth: true
        model: counterModel
        numberPadding: bulkCounterDialog.numberPadding
        stepperHeader: LocaleText.tr("Set next")
        minimumQuantity: 1
        maximumQuantity: 999999999
        onQuantityModified: function(index, nextQuantity) {
          counterModel.setProperty(index, "quantity", nextQuantity)
          bulkCounterDialog.revision += 1
        }
      }

      Controls.Label { text: LocaleText.tr("Reason") }
      Controls.TextField {
        id: reasonField
        Layout.fillWidth: true
      }

      Kirigami.InlineMessage {
        Layout.fillWidth: true
        type: Kirigami.MessageType.Warning
        text: LocaleText.tr("This changes counters only. Existing print records are kept unchanged.")
      }

      DialogActions {
        Controls.Button {
          text: LocaleText.tr("Apply")
          icon.name: "dialog-ok"
          highlighted: true
          enabled: {
            bulkCounterDialog.revision
            return bulkCounterDialog.changedCount() > 0
          }
          onClicked: bulkCounterDialog.submit()
        }
      }
    }
  }
}
