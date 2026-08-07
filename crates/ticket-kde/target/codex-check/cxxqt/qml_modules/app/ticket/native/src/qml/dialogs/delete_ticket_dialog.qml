import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: deleteDialog
  required property var backend

  title: "Delete ticket"
  dialogWidth: 460
  property string ticketId: ""
  property string ticketName: ""

  function submit() {
    root.acceptResponse(backend.deleteTicket(ticketId), "Ticket deleted")
    deleteDialog.close()
  }

  contentItem: ColumnLayout {

    Controls.Label {
      Layout.fillWidth: true
      text: "Delete “" + deleteDialog.ticketName + "” and all of its saved history?"
      wrapMode: Text.WordWrap
    }

    DialogActions {
      DangerButton {
        text: "Delete"
        onClicked: deleteDialog.submit()
      }
    }
  }
}
