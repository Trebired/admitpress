import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Item {
  id: dialogActions

  default property alias actions: actionSlot.data
  readonly property int gap: Kirigami.Units.smallSpacing

  Layout.fillWidth: true
  implicitHeight: actionRow.implicitHeight + gap

  RowLayout {
    id: actionRow

    x: 0
    y: dialogActions.gap
    width: parent.width
    height: implicitHeight
    spacing: dialogActions.gap

    Item { Layout.fillWidth: true }
    RowLayout {
      id: actionSlot
      spacing: dialogActions.gap
    }
  }
}
