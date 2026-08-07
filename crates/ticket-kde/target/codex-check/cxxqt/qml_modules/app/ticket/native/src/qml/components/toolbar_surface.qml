import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Rectangle {
  id: toolbarSurface

  default property alias content: toolbarRow.data
  property alias spacing: toolbarRow.spacing

  implicitHeight: toolbarRow.implicitHeight + Kirigami.Units.smallSpacing * 2
  color: Kirigami.Theme.backgroundColor
  clip: true

  Kirigami.Separator {
    anchors.bottom: parent.bottom
    anchors.left: parent.left
    anchors.right: parent.right
  }

  RowLayout {
    id: toolbarRow

    anchors.fill: parent
    anchors.margins: Kirigami.Units.smallSpacing
    spacing: Kirigami.Units.smallSpacing
  }
}
