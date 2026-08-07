import QtQuick
import org.kde.kirigami as Kirigami

Kirigami.GlobalDrawer {
  property var mainMenuAction
  property var ticketsAction
  property var settingsAction

  width: 190
  handleVisible: true
  collapseButtonVisible: false
  actions: [mainMenuAction, ticketsAction, settingsAction]

  header: Item {
    implicitWidth: 190
    implicitHeight: 52

    Image {
      anchors.horizontalCenter: parent.horizontalCenter
      anchors.verticalCenter: parent.verticalCenter
      source: Kirigami.ColorUtils.brightnessForColor(Kirigami.Theme.backgroundColor) === Kirigami.ColorUtils.Dark
      ? "qrc:/qt/qml/app/ticket/native/src/qml/assets/logo-dark.svg"
      : "qrc:/qt/qml/app/ticket/native/src/qml/assets/logo-light.svg"
      sourceSize.height: 26
      fillMode: Image.PreserveAspectFit
    }
  }
}
