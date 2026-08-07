import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Controls.ItemDelegate {
  id: listRow

  default property alias rowContent: rowLayout.data
  property bool showRowSeparator: true
  readonly property int scrollbarReserve: ListView.view
  ? Number(ListView.view.scrollbarReserve || 0)
  : 0

  width: ListView.view ? Math.max(0, ListView.view.width - scrollbarReserve) : implicitWidth
  implicitHeight: ListView.view && ListView.view.rowHeight
  ? ListView.view.rowHeight
  : Kirigami.Units.gridUnit * 2.2
  hoverEnabled: true
  leftPadding: Kirigami.Units.smallSpacing
  rightPadding: Kirigami.Units.smallSpacing
  topPadding: 0
  bottomPadding: 0

  contentItem: RowLayout {
    id: rowLayout

    spacing: Kirigami.Units.smallSpacing
  }

  Kirigami.Separator {
    anchors.left: parent.left
    anchors.right: parent.right
    anchors.bottom: parent.bottom
    visible: listRow.showRowSeparator
  }
}
