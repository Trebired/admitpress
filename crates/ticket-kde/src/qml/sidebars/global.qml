import QtQuick
import org.kde.kirigami as Kirigami
import "."

Kirigami.GlobalDrawer {
  id: sidebar

  property var mainMenuAction
  property var ticketsAction
  property var backupsAction
  property var release: null
  property var settingsAction
  property bool darkTheme: false

  width: 190
  handleVisible: true
  collapseButtonVisible: false
  actions: [mainMenuAction, ticketsAction, backupsAction, settingsAction]

  footer: GlobalSidebarFooter {
    release: sidebar.release
  }

  header: Item {
    implicitWidth: 190
    implicitHeight: 52

    Image {
      anchors.horizontalCenter: parent.horizontalCenter
      anchors.verticalCenter: parent.verticalCenter
      source: sidebar.darkTheme
      ? "qrc:/qt/qml/app/ticket/native/src/qml/assets/logo-dark.svg"
      : "qrc:/qt/qml/app/ticket/native/src/qml/assets/logo-light.svg"
      sourceSize.height: 26
      fillMode: Image.PreserveAspectFit
    }
  }
}
