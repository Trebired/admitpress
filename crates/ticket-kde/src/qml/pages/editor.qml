import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.kde.kirigami.layouts as KirigamiLayouts
import "../components"
import "../editor"

Kirigami.Page {
  id: editorPage

  property var root
  property var backend: root ? root.backendObject : null
  property var navigationController
  property string ticketId: ""
  property var summary: null
  property int selectedFieldIndex: -1
  property real canvasScale: 1
  property string backgroundPath: ""
  property string printerQueue: ""
  property bool snapToGrid: true
  property real gridTargetMm: 2.5
  property string borderStyle: "none"
  property real borderWidthMm: 0.2
  property string savedSnapshot: ""
  property bool savedSnapshotReady: false
  property bool samplePreviewEnabled: false
  property int samplePreviewSeed: 1
  property string activeTicketType: "classic"
  property int fieldPaletteRevision: 0
  property var activeFieldsModel: activeTicketType === "bulk" ? bulkFieldsModel : classicFieldsModel

  KirigamiLayouts.ColumnView.fillWidth: true
  Component.onCompleted: PageColumn.configure(KirigamiLayouts.ColumnView, {
      minimumWidth: Kirigami.Units.gridUnit * 48,
      preferredWidth: Kirigami.Units.gridUnit * 68,
      interactiveResizeEnabled: true
  })

  title: summary ? LocaleText.tr("Edit ") + summary.ticket.name : LocaleText.tr("Ticket editor")
  padding: 0

  ListModel { id: classicFieldsModel; dynamicRoles: true }
  ListModel { id: bulkFieldsModel; dynamicRoles: true }
  ListModel { id: subticketsModel; dynamicRoles: true }
  ListModel {
    id: classicFieldPaletteModel
    ListElement { label: "Static"; fieldKind: "static"; dynamicKind: ""; defaultText: "Text"; iconRole: "staticText" }
    ListElement { label: "Price"; fieldKind: "dynamic"; dynamicKind: "price"; defaultText: ""; iconRole: "price" }
    ListElement { label: "Date"; fieldKind: "dynamic"; dynamicKind: "date"; defaultText: ""; iconRole: "calendar" }
    ListElement { label: "Number"; fieldKind: "dynamic"; dynamicKind: "number"; defaultText: ""; iconRole: "number" }
  }
  ListModel {
    id: bulkFieldPaletteModel
    ListElement { label: "Static"; fieldKind: "static"; dynamicKind: ""; defaultText: "Text"; iconRole: "staticText" }
    ListElement { label: "Group name"; fieldKind: "dynamic"; dynamicKind: "bulk_name"; defaultText: ""; iconRole: "staticText" }
    ListElement { label: "Tickets"; fieldKind: "dynamic"; dynamicKind: "bulk_items"; defaultText: ""; iconRole: "groupItems" }
    ListElement { label: "Date"; fieldKind: "dynamic"; dynamicKind: "date"; defaultText: ""; iconRole: "calendar" }
  }

  EditorLogic {
    id: logic
    root: editorPage.root
    backend: editorPage.backend
    editor: editorPage
    props: propertiesPane
    canvas: canvasArea
    fieldsModel: editorPage.activeFieldsModel
    classicFieldsModel: classicFieldsModel
    bulkFieldsModel: bulkFieldsModel
    subticketsModel: subticketsModel
  }

  function prepareForTicket(ticketIdValue) {
    const nextTicketId = String(ticketIdValue || "")
    if (nextTicketId === "") return false
    ticketId = nextTicketId
    return logic.load()
  }

  function save(completion) {
    commitActiveInput()
    logic.persistence.save(completion)
  }

  function commitActiveInput() {
    if (root && root.forceActiveFocus) root.forceActiveFocus()
  }

  function markSavedState() {
    savedSnapshot = logic.persistence.snapshot()
    savedSnapshotReady = true
  }

  function hasUnsavedChanges() {
    commitActiveInput()
    return savedSnapshotReady && logic.persistence.snapshot() !== savedSnapshot
  }

  function addImportedImage(image) {
    logic.addImportedImage(image)
  }

  function setActiveTicketType(ticketType) {
    const nextType = ticketType === "bulk" ? "bulk" : "classic"
    if (activeTicketType === nextType) return
    activeTicketType = nextType
    const model = nextType === "bulk" ? bulkFieldsModel : classicFieldsModel
    logic.selection.selectSingle(model.count > 0 ? 0 : -1)
  }

  onBackRequested: event => {
    event.accepted = true
    navigationController.leaveEditorOrRun(editorPage, function() {
        root.pageStack.pop()
    })
  }

  actions: [
    Kirigami.Action {
      text: LocaleText.tr("Save")
      icon.name: AppIcons.pick("save")
      onTriggered: editorPage.save()
    }
  ]

  ColumnLayout {
    anchors.fill: parent
    spacing: 0

    EditorToolbar {
      editor: editorPage
      logic: logic
      Layout.fillWidth: true
    }

    RowLayout {
      spacing: 0
      layoutDirection: Qt.RightToLeft
      Layout.fillWidth: true
      Layout.fillHeight: true

      EditorPropertiesPane {
        id: propertiesPane
        root: editorPage.root
        editor: editorPage
        logic: logic
        fieldsModel: editorPage.activeFieldsModel
        fieldPaletteModel: editorPage.activeTicketType === "bulk" ? bulkFieldPaletteModel : classicFieldPaletteModel
        Layout.preferredWidth: 340
        Layout.minimumWidth: 280
        Layout.maximumWidth: 400
        Layout.fillWidth: false
        Layout.fillHeight: true
        LayoutMirroring.enabled: false
      }

      EditorCanvas {
        id: canvasArea
        root: editorPage.root
        editor: editorPage
        logic: logic
        fieldsModel: editorPage.activeFieldsModel
        Layout.minimumWidth: 420
        Layout.fillWidth: true
        Layout.fillHeight: true
        LayoutMirroring.enabled: false
      }
    }
  }

  Shortcut {
    sequences: [StandardKey.Copy]
    enabled: editorPage.visible
    onActivated: logic.copySelectedFields()
  }

  Shortcut {
    sequences: [StandardKey.Paste]
    enabled: editorPage.visible
    onActivated: logic.pasteFields()
  }
}
