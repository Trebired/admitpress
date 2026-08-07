import QtQuick
import QtQuick.Dialogs
import "../components"

FileDialog {
  id: imageDialog

  property var root
  property var backend: root ? root.backendObject : null
  property string ticketId: ""

  title: LocaleText.tr("Select image")
  nameFilters: [LocaleText.tr("Images") + " (*.svg *.png *.jpg *.jpeg *.gif *.bmp *.webp)"]
  fileMode: FileDialog.OpenFile

  function openFor(ticketIdValue) {
    ticketId = ticketIdValue
    open()
  }

  onAccepted: {
    const data = root.acceptResponse(
      backend.importImage(ticketId, selectedFile.toString()),
      LocaleText.tr("Image imported")
    )
    if (data && root.pageStack.currentItem && root.pageStack.currentItem.addImportedImage) {
      root.pageStack.currentItem.addImportedImage(data)
    }
  }
}
