import QtQuick
import QtQuick.Dialogs
import "../../../components"

FileDialog {
  id: ticketArchiveImportDialog

  property var root
  property var backend: root ? root.backendObject : null

  title: LocaleText.tr("Import ticket archive")
  nameFilters: [root.appDisplayName + " " + LocaleText.tr("archives") + " (*.tar.gz)"]
  fileMode: FileDialog.OpenFile

  onAccepted: {
    const data = root.acceptResponse(
      backend.importTicketArchive(selectedFile.toString()),
      LocaleText.tr("Ticket archive imported")
    )
    if (data && data.ticket_id) {
      root.openEditor(data.ticket_id)
    }
  }
}
