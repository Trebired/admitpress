import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"
import "."

EditorPropertySection {
  id: ticketSection

  property var root
  property var editor
  property var logic

  property alias ticketName: nameField.text
  property alias widthValue: widthField.realValue
  property alias heightValue: heightField.realValue
  property alias dpiValue: dpiField.value
  property alias borderWidthValue: borderWidthField.realValue

  function applyTicketSizeChange() {
    logic.normalizeFieldsToTicket()
    if (logic.canvas) logic.canvas.updateScale()
  }

  title: "Ticket"

  EditorFormField {
    label: "Name"
    Controls.TextField { id: nameField; Layout.fillWidth: true }
  }

  Controls.ButtonGroup { id: editorOrientationGroup }

  EditorFormField {
    label: "Orientation"
    RowLayout {
      Layout.fillWidth: true
      spacing: Kirigami.Units.smallSpacing
      Controls.Button {
        Layout.fillWidth: true
        text: LocaleText.tr("Vertical")
        icon.name: "view-preview"
        checkable: true
        checked: logic ? logic.isVerticalOrientation() : true
        Controls.ButtonGroup.group: editorOrientationGroup
        onClicked: logic.setOrientation(true)
      }
      Controls.Button {
        Layout.fillWidth: true
        text: LocaleText.tr("Horizontal")
        icon.name: "view-split-left-right"
        checkable: true
        checked: logic ? !logic.isVerticalOrientation() : false
        Controls.ButtonGroup.group: editorOrientationGroup
        onClicked: logic.setOrientation(false)
      }
    }
  }

  RowLayout {
    Layout.fillWidth: true
    spacing: Kirigami.Units.smallSpacing
    EditorFormField {
      label: "Width, cm"
      NumberInput {
        id: widthField
        Layout.fillWidth: true
        realFrom: 1.0
        realTo: 100.0
        realValue: 5.0
        onValueModified: ticketSection.applyTicketSizeChange()
      }
    }
    EditorFormField {
      label: "Height, cm"
      NumberInput {
        id: heightField
        Layout.fillWidth: true
        realFrom: 1.0
        realTo: 100.0
        realValue: 7.0
        onValueModified: ticketSection.applyTicketSizeChange()
      }
    }
  }

  RowLayout {
    Layout.fillWidth: true
    spacing: Kirigami.Units.smallSpacing
    EditorFormField {
      label: "Border"
      Controls.ComboBox {
        Layout.fillWidth: true
        model: ["none", "solid", "dashed", "dotted"]
        currentIndex: logic ? logic.borderStyleIndex() : 0
        onActivated: editor.borderStyle = currentText
      }
    }
    EditorFormField {
      label: "Line, mm"
      NumberInput {
        id: borderWidthField
        Layout.fillWidth: true
        enabled: editor && editor.borderStyle !== "none"
        realFrom: 0.1
        realTo: 5.0
        realValue: 0.2
        onValueModified: editor.borderWidthMm = realValue
      }
    }
  }

  Controls.SpinBox {
    id: dpiField
    visible: false
    from: 100
    to: 1200
    value: 203
  }
}
