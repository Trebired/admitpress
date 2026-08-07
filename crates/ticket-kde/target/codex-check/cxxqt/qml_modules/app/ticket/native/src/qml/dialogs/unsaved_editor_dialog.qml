import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: unsavedEditorDialog

  property var editorPage: null
  property var leaveAction: null

  title: LocaleText.tr("Unsaved changes")
  dialogWidth: 460

  function openWithContext(page, action) {
    clearPending()
    editorPage = page
    leaveAction = action
    open()
  }

  function clearPending() {
    editorPage = null
    leaveAction = null
  }

  function discardAndLeave() {
    const action = leaveAction
    close()
    clearPending()
    if (action) action()
  }

  function saveAndLeave() {
    const page = editorPage
    const action = leaveAction
    close()
    clearPending()
    if (!page || !page.save) return
    page.save(function(saved) {
        if (saved && action) action()
    })
  }

  onClosed: {
    if (!visible) clearPending()
  }

  contentItem: ColumnLayout {
    spacing: Kirigami.Units.largeSpacing

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("You have unsaved editor changes.")
    }

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("Save before leaving, or discard the changes and continue.")
      wrapMode: Text.WordWrap
    }

    DialogActions {
      DangerButton {
        text: LocaleText.tr("Discard")
        onClicked: unsavedEditorDialog.discardAndLeave()
      }

      Controls.Button {
        text: LocaleText.tr("Save and leave")
        icon.name: "document-save"
        highlighted: true
        onClicked: unsavedEditorDialog.saveAndLeave()
      }
    }
  }
}
