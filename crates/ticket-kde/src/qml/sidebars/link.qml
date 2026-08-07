import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Controls.Label {
  id: sidebarLink

  property string url: ""

  visible: url !== "" && text !== ""
  color: linkArea.containsMouse ? Kirigami.Theme.linkColor : Kirigami.Theme.disabledTextColor
  font.pointSize: Kirigami.Theme.smallFont.pointSize
  font.underline: linkArea.containsMouse

  MouseArea {
    id: linkArea

    anchors.fill: parent
    hoverEnabled: true
    cursorShape: Qt.PointingHandCursor
    onClicked: Qt.openUrlExternally(sidebarLink.url)
  }
}
