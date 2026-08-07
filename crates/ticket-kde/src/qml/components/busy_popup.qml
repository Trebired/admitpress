import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "."

Controls.Popup {
  id: busyPopup

  required property var appRoot

  parent: appRoot.overlay
  z: 1000
  x: Math.round((appRoot.width - width) / 2)
  y: Math.round((appRoot.height - height) / 2)
  width: Math.min(appRoot.width - Kirigami.Units.gridUnit * 2, 420)
  height: 112
  modal: true
  focus: true
  closePolicy: Controls.Popup.NoAutoClose
  visible: appRoot.busy
  background: Rectangle {
    color: Kirigami.Theme.backgroundColor
    border.color: Kirigami.Theme.disabledTextColor
    radius: 6
  }
  contentItem: Item {
    id: busyContentFrame

    property int gap: Kirigami.Units.largeSpacing

    RowLayout {
      x: busyContentFrame.gap
      y: busyContentFrame.gap
      width: busyContentFrame.width - busyContentFrame.gap * 2
      height: busyContentFrame.height - busyContentFrame.gap * 2
      spacing: busyContentFrame.gap

      Controls.BusyIndicator {
        running: appRoot.busy
        Layout.preferredWidth: Kirigami.Units.iconSizes.medium
        Layout.preferredHeight: Kirigami.Units.iconSizes.medium
      }

      Controls.Label {
        Layout.fillWidth: true
        text: appRoot.busyText
        wrapMode: Text.WordWrap
      }

      Controls.Button {
        visible: appRoot.busyOperation === "fix_printer"
        text: LocaleText.tr("Cancel")
        icon.name: AppIcons.pick("cancel")
        onClicked: appRoot.cancelBackendBusy()
      }
    }
  }
}
