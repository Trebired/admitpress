import QtQuick.Dialogs

MessageDialog {
  id: errorDialog
  required property var root
  title: root ? root.appDisplayName + " error" : "Application error"
  buttons: MessageDialog.Ok
  text: ""
}
