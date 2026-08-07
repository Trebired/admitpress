import QtQuick
import QtQuick.Dialogs
import "../../../../components"

FileDialog {
  id: ticketArchiveExportDialog

  property var root
  property var backend: root ? root.backendObject : null
  property string ticketId: ""
  property string ticketName: ""

  title: LocaleText.tr("Export ticket archive")
  nameFilters: [root.appDisplayName + " " + LocaleText.tr("archives") + " (*.tar.gz)"]
  fileMode: FileDialog.SaveFile
  defaultSuffix: "tar.gz"
  acceptLabel: LocaleText.tr("Export")

  function archiveFileName() {
    const cleaned = String(ticketName || "ticket")
    .trim()
    .replace(/[\/\\\0]/g, "-")
    .replace(/\s+/g, "-")
    .replace(/[^A-Za-z0-9._-]/g, "-")
    .replace(/-+/g, "-")
    .replace(/^[.-]+|[.-]+$/g, "")
    const baseName = cleaned.length > 0 ? cleaned : "ticket"
    return baseName + ".tar.gz"
  }

  function exportDirectory() {
    const dataDir = root && root.appState ? String(root.appState.data_dir || "") : ""
    return dataDir.length > 0 ? dataDir + "/exports" : ""
  }

  function openFor(ticketIdValue, ticketNameValue) {
    ticketId = String(ticketIdValue)
    ticketName = String(ticketNameValue || "")
    const directory = exportDirectory()
    currentFile = root.fileUrl(directory.length > 0 ? directory + "/" + archiveFileName() : archiveFileName())
    selectedFile = currentFile
    open()
  }

  onAccepted: {
    const target = selectedFile && selectedFile.toString()
    ? selectedFile.toString()
    : currentFile.toString()
    root.acceptResponse(
      backend.exportTicketArchive(ticketId, target),
      LocaleText.tr("Ticket archive exported")
    )
  }
}
