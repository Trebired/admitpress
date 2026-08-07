import QtQuick
import QtQuick.Dialogs
import "../../components"

FileDialog {
  id: dayExportDialog

  property var root
  property var backend: root ? root.backendObject : null
  property string ticketId: ""
  property string ticketName: ""
  property var localDates: []
  property string exportFormat: "xlsx"

  title: LocaleText.tr("Export days")
  nameFilters: [
    exportFormat === "ods"
    ? LocaleText.tr("OpenDocument spreadsheet") + " (*.ods)"
    : LocaleText.tr("Excel workbook") + " (*.xlsx)"
  ]
  fileMode: FileDialog.SaveFile
  defaultSuffix: exportFormat
  acceptLabel: LocaleText.tr("Export")

  function exportFileName() {
    const cleaned = String(ticketName || "ticket")
    .trim()
    .replace(/[\/\\\0]/g, "-")
    .replace(/\s+/g, "-")
    .replace(/[^A-Za-z0-9._-]/g, "-")
    .replace(/-+/g, "-")
    .replace(/^[.-]+|[.-]+$/g, "")
    const baseName = cleaned.length > 0 ? cleaned : "ticket"
    return baseName + "-days." + defaultSuffix
  }

  function openFor(ticketIdValue, ticketNameValue, localDatesValue, formatValue) {
    ticketId = String(ticketIdValue || "")
    ticketName = String(ticketNameValue || "")
    localDates = localDatesValue || []
    exportFormat = formatValue === "ods" ? "ods" : "xlsx"
    const dataDir = root && root.appState ? String(root.appState.data_dir || "") : ""
    const directory = dataDir.length > 0 ? dataDir + "/exports" : ""
    currentFile = root.fileUrl(directory.length > 0 ? directory + "/" + exportFileName() : exportFileName())
    selectedFile = currentFile
    open()
  }

  onAccepted: {
    const target = selectedFile && selectedFile.toString()
    ? selectedFile.toString()
    : currentFile.toString()
    root.acceptResponse(
      backend.exportDays(ticketId, JSON.stringify(localDates), target),
      LocaleText.tr("Days exported")
    )
  }
}
