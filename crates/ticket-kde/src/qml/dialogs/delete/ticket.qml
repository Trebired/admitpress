import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../../components"

AppDialog {
  id: deleteDialog
  required property var backend

  title: LocaleText.tr("Delete ticket")
  dialogWidth: 460
  property string ticketId: ""
  property string ticketName: ""

  function submit() {
    root.acceptResponse(backend.deleteTicket(ticketId), LocaleText.tr("Ticket deleted"))
    deleteDialog.close()
  }

  contentItem: ColumnLayout {
    spacing: deleteDialog.contentSpacing

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("Delete ticket prompt prefix")
      + deleteDialog.ticketName
      + LocaleText.tr("Delete ticket prompt suffix")
      wrapMode: Text.WordWrap
    }

    DialogActions {
      DangerButton {
        text: LocaleText.tr("Delete")
        onClicked: deleteDialog.submit()
      }
    }
  }
}
