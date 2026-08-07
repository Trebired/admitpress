pragma ComponentBehavior: Bound
import QtQuick
import org.kde.kirigami as Kirigami
import org.kde.kirigami.layouts as KirigamiLayouts
import app.ticket.native
import "../components"
import "../pages"
import "../dialogs"
import "../sidebars"
Kirigami.ApplicationWindow {
  id: root
  width: 1180
  height: 760
  minimumWidth: 900
  minimumHeight: 600
  visible: true
  title: appDisplayName
  property string appDisplayName: Qt.application.name || "Application"
  property var appState: defaultAppState()
  property bool forceCloseWithoutEditorGuard: false
  property bool busy: false
  property string busyText: ""
  property string busyOperation: ""
  property var pendingBusyCompletion: null
  property alias backendObject: backend
  property alias addTicketDialogObject: dialogRegistry.addTicketDialogObject
  property alias settingsDialogObject: dialogRegistry.settingsDialogObject
  property alias sudoFixDialogObject: dialogRegistry.sudoFixDialogObject
  property alias backupsDialogObject: dialogRegistry.backupsDialogObject
  property alias updateDialogObject: dialogRegistry.updateDialogObject
  property alias deleteDialogObject: dialogRegistry.deleteDialogObject
  property alias dataDirectoryDialogObject: dialogRegistry.dataDirectoryDialogObject
  property alias ticketArchiveExportSelectDialogObject: dialogRegistry.ticketArchiveExportSelectDialogObject
  property alias ticketArchiveExportDialogObject: dialogRegistry.ticketArchiveExportDialogObject
  property alias ticketArchiveImportDialogObject: dialogRegistry.ticketArchiveImportDialogObject
  property alias imageDialogObject: dialogRegistry.imageDialogObject
  property alias unsavedEditorDialogObject: dialogRegistry.unsavedEditorDialogObject
  property alias counterDialogObject: dialogRegistry.counterDialogObject
  property alias pricesDialogObject: dialogRegistry.pricesDialogObject
  property alias bulkCounterDialogObject: dialogRegistry.bulkCounterDialogObject
  property alias dayDetailDialogObject: dialogRegistry.dayDetailDialogObject
  property alias monthDetailDialogObject: dialogRegistry.monthDetailDialogObject
  property alias dayExportDialogObject: dialogRegistry.dayExportDialogObject
  property alias printOperationDialogObject: dialogRegistry.printOperationDialogObject
  property alias deleteDayDialogObject: dialogRegistry.deleteDayDialogObject
  property alias revertLastDialogObject: dialogRegistry.revertLastDialogObject
  property alias errorDialogObject: dialogRegistry.errorDialogObject
  readonly property var backupSchedulerObject: backgroundTasks.backups
  readonly property bool darkTheme: Kirigami.ColorUtils.brightnessForColor(
    Kirigami.Theme.backgroundColor) === Kirigami.ColorUtils.Dark
  QtObject {
    id: mainRefs
    property var windowObject: root
    property var backendObject: backend
    property var addTicketActionObject: addTicketAction
    property var ticketsActionObject: ticketsAction
    property var settingsActionObject: settingsAction
  }
  function defaultAppState() {
    return {
      tickets: [],
      printer_queues: [],
      selected_printer_queue: "",
      development_mode: true,
      data_dir: "data",
      main_currency: "",
      available_currencies: []
    }
  }

  function parseResponse(text) {
    try {
      return JSON.parse(text)
    } catch (error) {
      root.errorDialogObject.text = LocaleText.tr("The application returned invalid data: ")
      + error
      root.errorDialogObject.open()
      return { ok: false, error: String(error) }
    }
  }

  function acceptResponse(text, successMessage) {
    const response = parseResponse(text)
    if (!response.ok) {
      root.errorDialogObject.text = response.error || "Unknown error"
      root.errorDialogObject.open()
      return null
    }
    if (successMessage) showPassiveNotification(successMessage)
    return response.data
  }

  function clearBusy() {
    pendingBusyCompletion = null
    busyOperation = ""
    busyText = ""
    busy = false
  }

  function runBackendBusy(message, operationName, start, completion) {
    if (busy) return false
    busyText = message
    busyOperation = operationName
    pendingBusyCompletion = completion
    busy = true
    try {
      start()
    } catch (error) {
      const pendingCompletion = pendingBusyCompletion
      clearBusy()
      if (pendingCompletion) pendingCompletion(JSON.stringify({ ok: false, error: String(error) }))
      return false
    }
    return true
  }

  function completeBackendBusy(operationName, responseJson) {
    if (!busy || operationName !== busyOperation) return
    const completion = pendingBusyCompletion
    clearBusy()
    if (completion) completion(responseJson)
  }

  function cancelBackendBusy() {
    if (!busy) return
    if (busyOperation === "fix_printer") {
      root.acceptResponse(backend.cancelPrinterFix(), LocaleText.tr("Printer fix cancelled"))
      clearBusy()
    }
  }

  function routeAsyncResponse(payload) {
    const operation = payload.operation || ""
    const responseJson = payload.response_json || ""
    if (operation === "printer_diagnostics") {
      root.settingsDialogObject.handlePrinterDiagnosticsResponse(responseJson, payload.request_id || "")
      return
    }
    if (backgroundTasks.handleResponse(operation, responseJson)) return
    if (operation === "refresh_exchange_rate") {
      root.acceptResponse(responseJson)
      const page = root.pageStack.currentItem
      if (page && page.refreshSaleCurrencyDisplay) page.refreshSaleCurrencyDisplay()
      return
    }
    root.completeBackendBusy(operation, responseJson)
  }

  function reloadState() {
    backend.refresh()
  }

  function fileUrl(path) {
    return path ? encodeURI("file://" + path) : ""
  }

  function selectedPrinterQueue() {
    return root.appState.selected_printer_queue || ""
  }

  function ticketNextSummary(summary) {
    if (!summary || !summary.subtickets || summary.subtickets.length === 0) return ""
    const values = []
    for (let index = 0; index < summary.subtickets.length && index < 3; ++index) {
      const subticket = summary.subtickets[index]
      const label = MoneyFormat.format(subticket.amount_minor, subticket.currency)
      const number = TextFormat.displayTicketNumber(subticket.next_number_today, summary.ticket.number_padding)
      values.push(label + " " + number)
    }
    if (summary.subtickets.length > 3) values.push("...")
    return values.join(" | ")
  }

  function printerChoices() {
    const choices = [LocaleText.tr("No printer selected")]
    const queues = root.appState.printer_queues || []
    for (let index = 0; index < queues.length; ++index) {
      if (choices.indexOf(queues[index]) < 0) choices.push(queues[index])
    }
    const selected = root.selectedPrinterQueue()
    if (selected && choices.indexOf(selected) < 0) choices.push(selected)
    return choices
  }

  function printerChoiceIndex() {
    const selected = root.selectedPrinterQueue()
    if (!selected) return 0
    const choices = root.printerChoices()
    const index = choices.indexOf(selected)
    return index < 0 ? 0 : index
  }

  function createPage(component, properties) {
    const page = component.createObject(root.pageStack, properties || {})
    if (!page) {
      console.error("Failed to create page")
    }
    return page
  }

  function openHome() {
    if (root.pageStack.depth > 0) return
    const page = createPage(homePageComponent)
    if (page) root.pageStack.push(page)
  }

  function openEditor(ticketId) {
    navigation.openEditor(ticketId)
  }

  function openPrint(ticketId) {
    navigation.openPrint(ticketId)
  }

  function openMainMenu() {
    navigation.openMainMenu()
  }

  onClosing: function(closeEvent) {
    navigation.handleClosing(closeEvent)
  }

  TicketBackend {
    id: backend
    onStateJsonChanged: {
      try {
        root.appState = JSON.parse(backend.stateJson)
      } catch (error) {
        root.appState = defaultAppState()
      }
      if (!root.appState.main_currency) {
        backend.ensureMainCurrency(Qt.locale().name)
      }
    }

    onAsyncResponseJsonChanged: {
      const responseJson = backend.asyncResponseJson
      if (!responseJson) return
      try {
        const payload = JSON.parse(responseJson)
        root.routeAsyncResponse(payload)
      } catch (error) {
        root.completeBackendBusy(
          root.busyOperation,
          JSON.stringify({ ok: false, error: "The application returned invalid async data: " + error })
        )
      }
    }
  }
  Kirigami.Action {
    id: addTicketAction
    text: LocaleText.tr("Create")
    icon.name: AppIcons.pick("add")
    onTriggered: root.addTicketDialogObject.open()
  }
  Kirigami.Action {
    id: mainMenuAction
    text: LocaleText.tr("Main menu")
    icon.name: AppIcons.pick("home")
    enabled: root.pageStack.depth > 1
    onTriggered: root.openMainMenu()
  }
  Kirigami.Action {
    id: exportTicketAction
    text: LocaleText.tr("Export")
    icon.name: AppIcons.pick("export")
    enabled: root.appState.tickets && root.appState.tickets.length > 0
    onTriggered: root.ticketArchiveExportSelectDialogObject.open()
  }
  Kirigami.Action {
    id: importTicketAction
    text: LocaleText.tr("Import")
    icon.name: AppIcons.pick("import")
    onTriggered: root.ticketArchiveImportDialogObject.open()
  }
  Kirigami.Action {
    id: ticketsAction
    text: LocaleText.tr("Tickets")
    icon.name: AppIcons.pick("folder")
    children: [addTicketAction, importTicketAction, exportTicketAction]
  }
  Kirigami.Action {
    id: backupsAction
    text: LocaleText.tr("Backups")
    icon.name: AppIcons.pick("backup")
    onTriggered: root.backupsDialogObject.open()
  }
  Kirigami.Action {
    id: settingsAction
    text: LocaleText.tr("Settings")
    icon.name: AppIcons.pick("settings")
    onTriggered: root.settingsDialogObject.open()
  }
  pageStack {
    defaultColumnWidth: Kirigami.Units.gridUnit * 24
    Component.onCompleted: {
      PageColumn.resizeMode(root.pageStack.columnView, KirigamiLayouts.ColumnView.DynamicColumns)
      root.openHome()
    }
  }
  onDarkThemeChanged: backend.setWindowIcon(root.darkTheme)

  Component.onCompleted: {
    backend.setWindowIcon(root.darkTheme)
    LocaleText.backend = backend
    AppIcons.backend = backend
    MoneyFormat.backend = backend
    backend.initialize()
  }
  MainNavigation {
    id: navigation
    root: mainRefs.windowObject
    editorPageComponent: editorPageComponent
    printPageComponent: printPageComponent
    unsavedEditorDialog: root.unsavedEditorDialogObject
  }
  globalDrawer: GlobalSidebar {
    id: sidebar
    darkTheme: root.darkTheme
    release: backgroundTasks.release.status
    mainMenuAction: mainMenuAction
    ticketsAction: ticketsAction
    backupsAction: backupsAction
    settingsAction: settingsAction
  }
  BusyPopup {
    appRoot: mainRefs.windowObject
  }
  BackgroundTasks {
    id: backgroundTasks
    root: mainRefs.windowObject
    backend: mainRefs.backendObject
    onUpdateDetected: function(release) {
      root.updateDialogObject.openFor(release)
    }
  }
  Component {
    id: homePageComponent
    HomePage {
      root: mainRefs.windowObject
      addTicketAction: mainRefs.addTicketActionObject
      ticketsAction: mainRefs.ticketsActionObject
      settingsAction: mainRefs.settingsActionObject
    }
  }
  Component {
    id: editorPageComponent
    EditorPage {
      root: mainRefs.windowObject
      backend: mainRefs.backendObject
      navigationController: navigation
    }
  }
  Component {
    id: printPageComponent
    PrintPage {
      root: mainRefs.windowObject
      backend: mainRefs.backendObject
    }
  }
  DialogRegistry {
    id: dialogRegistry
    root: mainRefs.windowObject
    backend: mainRefs.backendObject
  }
}
