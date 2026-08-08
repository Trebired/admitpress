import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: addTicketDialog

  property var backend: root ? root.backendObject : null
  property bool verticalOrientation: true
  property string selectedQueue: ""
  readonly property bool usesPrinter: selectedQueue.trim() !== ""
  property var printerProfiles: []

  title: LocaleText.tr("Add new ticket")
  dialogWidth: 420

  function addTicketSizeCm(input, fallback) {
    const value = Number(input.realValue)
    return Number.isFinite(value) && value > 0 ? value : fallback
  }

  function applyPrinterSize() {
    if (!usesPrinter) return
    const profile = PrinterProfiles.find(printerProfiles, selectedQueue)
    addWidth.realValue = profile ? profile.width_mm / 10 : 5.0
    addHeight.realValue = profile ? profile.height_mm / 10 : 7.0
    verticalOrientation = addHeight.realValue >= addWidth.realValue
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
    selectedQueue = ""
    addWidth.realValue = 5.0
    addHeight.realValue = 7.0
    printerProfiles = PrinterProfiles.fetch(root, backend)
  }

  function submit() {
    const data = root.acceptResponse(backend.createTicket(
        addName.text,
        addWidth.realValue,
        addHeight.realValue,
        addTicketDialog.selectedQueue
      ), LocaleText.tr("Ticket created"))
    if (data) {
      addName.text = ""
      addTicketDialog.close()
      root.openEditor(data.ticket_id)
    }
  }

  contentItem: ColumnLayout {
    spacing: addTicketDialog.contentSpacing

    Controls.Label { text: LocaleText.tr("Name") }
    Controls.TextField {
      id: addName
      Layout.fillWidth: true
      placeholderText: LocaleText.tr("Ticket name")
    }

    Controls.Label { text: LocaleText.tr("Printer") }
    Controls.ComboBox {
      id: addPrinter
      Layout.fillWidth: true
      model: root.printerChoices()
      currentIndex: 0
      onActivated: function(index) {
        addTicketDialog.selectedQueue = index <= 0 ? "" : addPrinter.textAt(index)
        addTicketDialog.applyPrinterSize()
      }
    }
    Controls.Label {
      Layout.fillWidth: true
      wrapMode: Text.WordWrap
      color: Kirigami.Theme.disabledTextColor
      text: addTicketDialog.usesPrinter
      ? LocaleText.tr("The ticket takes the size calibrated for this printer.")
      : LocaleText.tr("Calibrate a printer in Settings first, or set the size by hand below.")
    }

    Controls.ButtonGroup { id: addOrientationGroup }
    Controls.Label {
      text: LocaleText.tr("Orientation")
      visible: !addTicketDialog.usesPrinter
    }
    RowLayout {
      Layout.fillWidth: true
      visible: !addTicketDialog.usesPrinter
      Controls.Button {
        Layout.fillWidth: true
        text: LocaleText.tr("Vertical")
        icon.name: AppIcons.pick("preview")
        checkable: true
        checked: addTicketDialog.verticalOrientation
        Controls.ButtonGroup.group: addOrientationGroup
        onClicked: addTicketDialog.setOrientation(true)
      }
      Controls.Button {
        Layout.fillWidth: true
        text: LocaleText.tr("Horizontal")
        icon.name: AppIcons.pick("splitView")
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
          enabled: !addTicketDialog.usesPrinter
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
          enabled: !addTicketDialog.usesPrinter
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
        icon.name: AppIcons.pick("ok")
        highlighted: true
        onClicked: addTicketDialog.submit()
      }
    }
  }
}
