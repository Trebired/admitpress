import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: addTicketDialog

  property var backend: root ? root.backendObject : null
  property bool verticalOrientation: true

  title: LocaleText.tr("Add new ticket")
  dialogWidth: 420

  function addTicketSizeCm(input, fallback) {
    const value = Number(input.realValue)
    return Number.isFinite(value) && value > 0 ? value : fallback
  }

  function setOrientation(vertical) {
    verticalOrientation = vertical
    const width = addTicketSizeCm(addWidth, 5)
    const height = addTicketSizeCm(addHeight, 7)
    const shortSide = Math.min(width, height)
    const longSide = Math.max(width, height)
    addWidth.realValue = vertical ? shortSide : longSide
    addHeight.realValue = vertical ? longSide : shortSide
  }

  onOpened: {
    verticalOrientation = true
    addWidth.realValue = 5.0
    addHeight.realValue = 7.0
  }

  function submit() {
    const data = root.acceptResponse(backend.createTicket(
        addName.text,
        addWidth.realValue,
        addHeight.realValue
      ), LocaleText.tr("Ticket created"))
    if (data) {
      addName.text = ""
      addTicketDialog.close()
      root.openEditor(data.ticket_id)
    }
  }

  contentItem: ColumnLayout {

    Controls.Label { text: LocaleText.tr("Name") }
    Controls.TextField {
      id: addName
      Layout.fillWidth: true
      placeholderText: LocaleText.tr("Ticket name")
    }

    Controls.ButtonGroup { id: addOrientationGroup }
    Controls.Label { text: LocaleText.tr("Orientation") }
    RowLayout {
      Layout.fillWidth: true
      Controls.Button {
        Layout.fillWidth: true
        text: LocaleText.tr("Vertical")
        icon.name: "view-preview"
        checkable: true
        checked: addTicketDialog.verticalOrientation
        Controls.ButtonGroup.group: addOrientationGroup
        onClicked: addTicketDialog.setOrientation(true)
      }
      Controls.Button {
        Layout.fillWidth: true
        text: LocaleText.tr("Horizontal")
        icon.name: "view-split-left-right"
        checkable: true
        checked: !addTicketDialog.verticalOrientation
        Controls.ButtonGroup.group: addOrientationGroup
        onClicked: addTicketDialog.setOrientation(false)
      }
    }

    RowLayout {
      Layout.fillWidth: true
      ColumnLayout {
        Layout.fillWidth: true
        Controls.Label { text: LocaleText.tr("Width, cm") }
        NumberInput {
          id: addWidth
          Layout.fillWidth: true
          realFrom: 1.0
          realTo: 100.0
          realValue: 5.0
        }
      }
      ColumnLayout {
        Layout.fillWidth: true
        Controls.Label { text: LocaleText.tr("Height, cm") }
        NumberInput {
          id: addHeight
          Layout.fillWidth: true
          realFrom: 1.0
          realTo: 100.0
          realValue: 7.0
        }
      }
    }

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("The new ticket starts empty. Add fields and prices in the editor.")
      wrapMode: Text.WordWrap
      color: Kirigami.Theme.disabledTextColor
    }

    DialogActions {
      Controls.Button {
        text: LocaleText.tr("OK")
        icon.name: "dialog-ok"
        highlighted: true
        onClicked: addTicketDialog.submit()
      }
    }
  }
}
