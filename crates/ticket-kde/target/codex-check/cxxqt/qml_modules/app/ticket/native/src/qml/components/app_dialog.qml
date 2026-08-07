import QtQuick
import QtQuick.Controls as Controls
import org.kde.kirigami as Kirigami

Controls.Dialog {
  id: appDialog

  property var root
  property int dialogWidth: 460
  property int dialogHeight: 0
  readonly property int dialogScreenHeight: root
  ? Math.max(0, root.height - Kirigami.Units.gridUnit * 2)
  : 0
  readonly property int dialogMaximumHeight: {
    if (dialogHeight <= 0) {
      return dialogScreenHeight
    }
    if (dialogScreenHeight <= 0) {
      return dialogHeight
    }
    return Math.min(dialogScreenHeight, dialogHeight)
  }

  parent: root ? root.overlay : null
  x: root ? Math.round((root.width - width) / 2) : 0
  y: root ? Math.round((root.height - height) / 2) : 0
  width: root ? Math.min(root.width - Kirigami.Units.gridUnit * 2, dialogWidth) : dialogWidth
  height: dialogMaximumHeight > 0
  ? Math.min(implicitHeight, dialogMaximumHeight)
  : implicitHeight
  modal: true
  padding: Kirigami.Units.largeSpacing
  standardButtons: Controls.Dialog.NoButton
}
