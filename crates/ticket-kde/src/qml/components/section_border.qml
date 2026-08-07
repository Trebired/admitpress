import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Item {
  id: sectionBorder

  property int verticalGap: Kirigami.Units.largeSpacing

  Layout.fillWidth: true
  Layout.preferredHeight: verticalGap * 2 + 1
  Layout.minimumHeight: 1

  Kirigami.Separator {
    anchors.left: parent.left
    anchors.right: parent.right
    anchors.verticalCenter: parent.verticalCenter
  }
}
