import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: calibrationDialog

  property var backend: root ? root.backendObject : null
  property var printerProfiles: []
  property string selectedQueue: ""
  readonly property bool hasQueue: selectedQueue.trim() !== ""
  readonly property bool printing: root && root.busy && root.busyOperation === "print_calibration_sheet"

  title: LocaleText.tr("Printer calibration")
  dialogWidth: 480

  function queueChoices() {
    const detected = root ? root.printerChoices().slice(1) : []
    const known = new Set(detected)
    const choices = detected.slice()
    for (const profile of printerProfiles) {
      if (!known.has(profile.queue)) {
        known.add(profile.queue)
        choices.push(profile.queue)
      }
    }
    return choices
  }

  function syncEditorsToQueue() {
    const profile = PrinterProfiles.find(printerProfiles, selectedQueue)
    pageWidth.realValue = profile ? profile.width_mm / 10 : 5.0
    pageHeight.realValue = profile ? profile.height_mm / 10 : 7.0
    offsetX.realValue = profile ? profile.offset_x_mm : 0
    offsetY.realValue = profile ? profile.offset_y_mm : 0
    scaleX.realValue = profile ? profile.scale_x_percent : 100
    scaleY.realValue = profile ? profile.scale_y_percent : 100
  }

  function selectQueue(queue) {
    selectedQueue = queue
    syncEditorsToQueue()
  }

  onOpened: {
    printerProfiles = PrinterProfiles.fetch(root, backend)
    const choices = queueChoices()
    const preferred = root.selectedPrinterQueue()
    const initial = preferred && choices.indexOf(preferred) >= 0 ? preferred : (choices[0] || "")
    selectQueue(initial)
  }

  function resetFields() {
    offsetX.realValue = 0
    offsetY.realValue = 0
    scaleX.realValue = 100
    scaleY.realValue = 100
  }

  function save() {
    if (!hasQueue) return
    const sizeSaved = root.acceptResponse(
      backend.setPrinterPageSize(selectedQueue, pageWidth.realValue, pageHeight.realValue)
    )
    if (!sizeSaved) return
    const calibrationSaved = root.acceptResponse(
      backend.setPrinterCalibration(
        selectedQueue, offsetX.realValue, offsetY.realValue, scaleX.realValue, scaleY.realValue
      ),
      LocaleText.tr("Printer calibration saved")
    )
    if (calibrationSaved) printerProfiles = PrinterProfiles.fetch(root, backend)
  }

  function currentCalibrationJson() {
    return JSON.stringify({
        width_cm: pageWidth.realValue,
        height_cm: pageHeight.realValue,
        offset_x_mm: offsetX.realValue,
        offset_y_mm: offsetY.realValue,
        scale_x_percent: scaleX.realValue,
        scale_y_percent: scaleY.realValue
    })
  }

  function printSheet() {
    if (!hasQueue) return
    root.runBackendBusy(LocaleText.tr("Printing calibration sheet"), "print_calibration_sheet", function() {
        backend.printCalibrationSheetAsync(selectedQueue, calibrationDialog.currentCalibrationJson())
      }, function(responseJson) {
        if (!calibrationDialog.opened) return
        root.acceptResponse(responseJson, LocaleText.tr("Calibration sheet sent"))
    })
  }

  contentItem: ColumnLayout {
    spacing: calibrationDialog.contentSpacing

    Controls.Label {
      Layout.fillWidth: true
      wrapMode: Text.WordWrap
      color: Kirigami.Theme.disabledTextColor
      text: LocaleText.tr("Print the sheet, measure it, and enter what came out.")
    }

    Controls.Label { text: LocaleText.tr("Printer queue") }
    Controls.ComboBox {
      id: queueCombo
      Layout.fillWidth: true
      model: calibrationDialog.queueChoices()
      currentIndex: Math.max(0, model.indexOf(calibrationDialog.selectedQueue))
      onActivated: calibrationDialog.selectQueue(textAt(currentIndex))
    }

    EmptyState {
      visible: !calibrationDialog.hasQueue
      text: LocaleText.tr("No printer queues detected.")
    }

    ColumnLayout {
      Layout.fillWidth: true
      visible: calibrationDialog.hasQueue
      spacing: calibrationDialog.contentSpacing

      SectionBorder {}

      GridLayout {
        Layout.fillWidth: true
        columns: 2
        columnSpacing: Kirigami.Units.smallSpacing
        rowSpacing: Kirigami.Units.smallSpacing

        Controls.Label { text: LocaleText.tr("Paper width, cm") }
        Controls.Label { text: LocaleText.tr("Paper height, cm") }
        NumberInput { id: pageWidth; Layout.fillWidth: true; realFrom: 1; realTo: 100 }
        NumberInput { id: pageHeight; Layout.fillWidth: true; realFrom: 1; realTo: 100 }
      }

      Controls.Button {
        Layout.fillWidth: true
        icon.name: AppIcons.pick("printer")
        text: calibrationDialog.printing
        ? LocaleText.tr("Printing...")
        : LocaleText.tr("Print calibration sheet")
        enabled: !calibrationDialog.printing
        onClicked: calibrationDialog.printSheet()
      }

      Controls.Label {
        Layout.fillWidth: true
        wrapMode: Text.WordWrap
        color: Kirigami.Theme.disabledTextColor
        text: LocaleText.tr("Measure the printed bars. If one came out short, raise its scale.")
      }

      SectionBorder {}

      GridLayout {
        Layout.fillWidth: true
        columns: 2
        columnSpacing: Kirigami.Units.smallSpacing
        rowSpacing: Kirigami.Units.smallSpacing

        Controls.Label { text: LocaleText.tr("Offset right, mm") }
        Controls.Label { text: LocaleText.tr("Offset down, mm") }
        NumberInput { id: offsetX; Layout.fillWidth: true; realFrom: -50; realTo: 50 }
        NumberInput { id: offsetY; Layout.fillWidth: true; realFrom: -50; realTo: 50 }

        Controls.Label { text: LocaleText.tr("Scale across, %") }
        Controls.Label { text: LocaleText.tr("Scale down, %") }
        NumberInput { id: scaleX; Layout.fillWidth: true; realFrom: 80; realTo: 120 }
        NumberInput { id: scaleY; Layout.fillWidth: true; realFrom: 80; realTo: 120 }
      }

      Controls.Button {
        Layout.fillWidth: true
        icon.name: AppIcons.pick("reset")
        text: LocaleText.tr("Reset calibration fields")
        onClicked: calibrationDialog.resetFields()
      }
    }

    DialogActions {
      Controls.Button {
        text: LocaleText.tr("Close")
        onClicked: calibrationDialog.close()
      }
      Controls.Button {
        text: LocaleText.tr("Save")
        icon.name: AppIcons.pick("save")
        highlighted: true
        enabled: calibrationDialog.hasQueue
        onClicked: calibrationDialog.save()
      }
    }
  }
}
