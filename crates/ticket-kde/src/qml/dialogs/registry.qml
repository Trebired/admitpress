import QtQuick

Item {
  id: dialogRegistry

  property var root
  property var backend
  property alias addTicketDialogObject: addTicketDialog
  property alias settingsDialogObject: settingsDialog
  property alias sudoFixDialogObject: sudoFixDialog
  property alias backupsDialogObject: backupsDialog
  property alias updateDialogObject: updateDialog
  property alias deleteDialogObject: deleteDialog
  property alias dataDirectoryDialogObject: dataDirectoryDialog
  property alias ticketArchiveExportSelectDialogObject: ticketArchiveExportSelectDialog
  property alias ticketArchiveExportDialogObject: ticketArchiveExportDialog
  property alias ticketArchiveImportDialogObject: ticketArchiveImportDialog
  property alias imageDialogObject: imageDialog
  property alias unsavedEditorDialogObject: unsavedEditorDialog
  property alias counterDialogObject: counterDialog
  property alias pricesDialogObject: pricesDialog
  property alias bulkCounterDialogObject: bulkCounterDialog
  property alias dayDetailDialogObject: dayDetailDialog
  property alias monthDetailDialogObject: monthDetailDialog
  property alias dayExportDialogObject: dayExportDialog
  property alias printOperationDialogObject: printOperationDialog
  property alias deleteDayDialogObject: deleteDayDialog
  property alias revertLastDialogObject: revertLastDialog
  property alias errorDialogObject: errorDialog

  AddTicketDialog { id: addTicketDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  SettingsDialog { id: settingsDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  SudoFixDialog { id: sudoFixDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  BackupsDialog { id: backupsDialog; root: dialogRegistry.root }
  UpdateDialog { id: updateDialog; root: dialogRegistry.root }
  DeleteTicketDialog { id: deleteDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  DataDirectoryDialog { id: dataDirectoryDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  TicketArchiveExportSelectDialog { id: ticketArchiveExportSelectDialog; root: dialogRegistry.root }
  TicketArchiveExportDialog { id: ticketArchiveExportDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  TicketArchiveImportDialog { id: ticketArchiveImportDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  ImageDialog { id: imageDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  UnsavedEditorDialog { id: unsavedEditorDialog; root: dialogRegistry.root }
  CounterDialog { id: counterDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  PricesDialog { id: pricesDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  BulkCounterDialog { id: bulkCounterDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  DayDetailDialog { id: dayDetailDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  MonthDetailDialog { id: monthDetailDialog; root: dialogRegistry.root }
  DayExportDialog { id: dayExportDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  PrintOperationDialog { id: printOperationDialog; root: dialogRegistry.root }
  DeleteDayDialog { id: deleteDayDialog; root: dialogRegistry.root; backend: dialogRegistry.backend }
  RevertLastDialog { id: revertLastDialog; root: dialogRegistry.root }
  ErrorDialog { id: errorDialog; root: dialogRegistry.root }
}
