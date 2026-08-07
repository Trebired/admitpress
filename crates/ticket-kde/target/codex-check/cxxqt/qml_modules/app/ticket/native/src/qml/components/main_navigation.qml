import QtQuick

QtObject {
  id: navigation

  property var root
  property var editorPageComponent
  property var printPageComponent
  property var unsavedEditorDialog

  function openEditor(ticketId) {
    leaveCurrentEditorOrRun(function() {
        const page = root.createPage(editorPageComponent)
        if (page && page.prepareForTicket(ticketId)) {
          root.pageStack.push(page)
        } else if (page) {
          page.destroy()
        }
    })
  }

  function openPrint(ticketId) {
    const printTicketId = String(ticketId || "")
    leaveCurrentEditorOrRun(function() {
        const page = root.createPage(printPageComponent)
        if (page && page.prepareForTicket(printTicketId)) {
          root.pageStack.push(page)
        } else if (page) {
          page.destroy()
        }
    })
  }

  function editorHasUnsavedChanges(page) {
    if (root.forceActiveFocus) root.forceActiveFocus()
    return pageHasUnsavedChanges(page)
  }

  function pageHasUnsavedChanges(page) {
    return !!(page && page.hasUnsavedChanges && page.hasUnsavedChanges())
  }

  function leaveEditorOrRun(page, action) {
    if (!editorHasUnsavedChanges(page)) {
      action()
      return true
    }
    unsavedEditorDialog.openWithContext(page, action)
    return false
  }

  function currentUnsavedEditorPage() {
    if (root.forceActiveFocus) root.forceActiveFocus()
    const stack = root.pageStack
    for (let index = stack.depth - 1; index >= 0; --index) {
      const page = stack.get(index)
      if (pageHasUnsavedChanges(page)) return page
    }
    return null
  }

  function leaveCurrentEditorOrRun(action) {
    const page = currentUnsavedEditorPage()
    if (!page) {
      action()
      return true
    }
    unsavedEditorDialog.openWithContext(page, action)
    return false
  }

  function openMainMenu() {
    leaveCurrentEditorOrRun(function() {
        root.pageStack.pop(null)
    })
  }

  function closeWithoutEditorGuard() {
    root.forceCloseWithoutEditorGuard = true
    root.close()
  }

  function handleClosing(closeEvent) {
    if (root.forceCloseWithoutEditorGuard) return
    const page = currentUnsavedEditorPage()
    if (!page) return
    closeEvent.accepted = false
    unsavedEditorDialog.openWithContext(page, function() {
        closeWithoutEditorGuard()
    })
  }
}
