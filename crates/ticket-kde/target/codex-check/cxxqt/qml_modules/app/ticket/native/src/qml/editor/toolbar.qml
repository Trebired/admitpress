import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Rectangle {
  id: editorToolbar

  property var editor
  property var logic
  readonly property int gap: Kirigami.Units.smallSpacing
  readonly property bool compact: width < Kirigami.Units.gridUnit * 34

  implicitHeight: toolbarRow.implicitHeight + gap * 2
  color: Kirigami.Theme.backgroundColor
  clip: true

  Kirigami.Separator {
    anchors.bottom: parent.bottom
    anchors.left: parent.left
    anchors.right: parent.right
  }

  RowLayout {
    id: toolbarRow

    x: editorToolbar.gap
    y: editorToolbar.gap
    width: parent.width - editorToolbar.gap * 2
    height: implicitHeight
    spacing: editorToolbar.gap

    Controls.ToolButton {
      icon.name: "edit-undo"
      text: LocaleText.tr("Undo")
      display: Controls.AbstractButton.IconOnly
      enabled: !!editorToolbar.logic && editorToolbar.logic.history.canUndo
      onClicked: editorToolbar.logic.history.undo()

      Controls.ToolTip.visible: hovered
      Controls.ToolTip.text: LocaleText.tr("Undo")
    }

    Controls.ToolButton {
      icon.name: "edit-redo"
      text: LocaleText.tr("Redo")
      display: Controls.AbstractButton.IconOnly
      enabled: !!editorToolbar.logic && editorToolbar.logic.history.canRedo
      onClicked: editorToolbar.logic.history.redo()

      Controls.ToolTip.visible: hovered
      Controls.ToolTip.text: LocaleText.tr("Redo")
    }

    Item {
      Layout.fillHeight: true
      Layout.preferredWidth: 1

      Kirigami.Separator {
        anchors.horizontalCenter: parent.horizontalCenter
        y: editorToolbar.gap
        width: 1
        height: parent.height - editorToolbar.gap * 2
      }
    }

    Controls.TabBar {
      Layout.preferredWidth: editorToolbar.compact ? 180 : 260
      currentIndex: editorToolbar.editor && editorToolbar.editor.activeTicketType === "bulk" ? 1 : 0

      Controls.TabButton {
        text: LocaleText.tr("Classic")
        onClicked: if (editorToolbar.editor) editorToolbar.editor.setActiveTicketType("classic")
      }

      Controls.TabButton {
        text: LocaleText.tr("Bulk ticket")
        onClicked: if (editorToolbar.editor) editorToolbar.editor.setActiveTicketType("bulk")
      }
    }

    Item {
      Layout.fillHeight: true
      Layout.preferredWidth: 1

      Kirigami.Separator {
        anchors.horizontalCenter: parent.horizontalCenter
        y: editorToolbar.gap
        width: 1
        height: parent.height - editorToolbar.gap * 2
      }
    }

    Controls.CheckBox {
      text: editorToolbar.compact ? "" : "Preview sample data"
      checked: !!editorToolbar.editor && editorToolbar.editor.samplePreviewEnabled
      onToggled: {
        editorToolbar.editor.samplePreviewEnabled = checked
        if (checked && editorToolbar.logic) editorToolbar.logic.refreshSamplePreview()
      }

      Controls.ToolTip.visible: editorToolbar.compact && hovered
      Controls.ToolTip.text: LocaleText.tr("Preview sample data")
    }

    Item { Layout.fillWidth: true }
  }
}
