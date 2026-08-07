import QtQuick
import QtQuick.Dialogs

FolderDialog {
  id: dataDirectoryDialog

  property var root
  property var backend: root ? root.backendObject : null

  title: root ? "Choose " + root.appDisplayName + " data directory" : "Choose data directory"
  onAccepted: root.acceptResponse(
    backend.setDataDirectory(selectedFolder.toString()),
    "Data directory changed"
  )
}
