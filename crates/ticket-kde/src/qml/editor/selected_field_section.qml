import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"
import "."

EditorPropertySection {
  id: selectedSection

  property var root
  property var editor
  property var logic
  property var fieldsModel

  title: "Selected field"

  function primaryField() {
    if (!editor || !fieldsModel) return null
    const index = Number(editor.selectedFieldIndex)
    if (index < 0 || index >= fieldsModel.count) return null
    return fieldsModel.get(index) || null
  }

  function hasSelection() {
    if (!editor || !fieldsModel) return false
    if (!primaryField()) return false
    return logic && logic.selection ? logic.selection.hasSelection() : true
  }

  function selectedCount() {
    if (!hasSelection()) return 0
    return logic && logic.selection ? logic.selection.selectedCount() : 1
  }

  function selectedField() {
    return hasSelection() ? primaryField() : null
  }

  function selectedFieldIs(kind) {
    const field = selectedField()
    return !!field && field.field_kind === kind
  }

  function labelForSelectedField() {
    const field = selectedField()
    if (!field) return ""
    if (selectedCount() > 1) return selectedCount() + LocaleText.tr(" fields selected")
    if (field.field_kind === "image") return LocaleText.tr("Image")
    if (field.field_kind === "static") return LocaleText.tr("Static text")
    return LocaleText.tr("Dynamic ") + field.dynamic_kind
  }

  function alignmentIndex(value, values) {
    const index = values.indexOf(value)
    return index < 0 ? 0 : index
  }

  function isBold() {
    const field = selectedField()
    return !!field && Number(field.font_weight) >= 700
  }

  function isItalic() {
    const field = selectedField()
    return !!field && field.font_italic === true
  }

  function applyToSelection(name, value) {
    if (!hasSelection()) return
    const indices = logic && logic.selection
    ? logic.selection.selectedIndices()
    : [editor.selectedFieldIndex]
    for (const index of indices) {
      fieldsModel.setProperty(index, name, value)
    }
  }

  Controls.Label {
    Layout.fillWidth: true
    visible: !selectedSection.hasSelection()
    text: LocaleText.tr("Select a field on the canvas.")
    wrapMode: Text.WordWrap
    color: Kirigami.Theme.disabledTextColor
  }

  ColumnLayout {
    visible: selectedSection.hasSelection()
    Layout.fillWidth: true
    spacing: Kirigami.Units.smallSpacing

    Controls.Label {
      Layout.fillWidth: true
      text: selectedSection.labelForSelectedField()
      elide: Text.ElideRight
      color: Kirigami.Theme.disabledTextColor
    }

    EditorFieldBoundsControls {
      logic: selectedSection.logic
    }

    EditorFormField {
      visible: selectedSection.selectedFieldIs("static")
      label: "Text"
      Controls.TextField {
        Layout.fillWidth: true
        text: selectedSection.hasSelection() ? selectedSection.selectedField().text : ""
        placeholderText: LocaleText.tr("Static text")
        onTextEdited: fieldsModel.setProperty(editor.selectedFieldIndex, "text", text)
      }
    }

    EditorFormField {
      visible: logic ? logic.selectedFieldIsImage() : false
      label: "Image"
      Controls.Label {
        Layout.fillWidth: true
        text: selectedSection.hasSelection() ? selectedSection.selectedField().text : ""
        elide: Text.ElideMiddle
        color: Kirigami.Theme.disabledTextColor
      }
    }

    EditorFormField {
      visible: logic ? logic.selectedFieldIsDate() : false
      label: "Date format"
      Controls.ComboBox {
        Layout.fillWidth: true
        model: [
          LocaleText.tr("Date"),
          LocaleText.tr("Date + time"),
          LocaleText.tr("Full date + time"),
          LocaleText.tr("ISO date"),
          LocaleText.tr("ISO date + time"),
          LocaleText.tr("Time only")
        ]
        currentIndex: logic && selectedSection.hasSelection()
        ? logic.dateFormatIndex(selectedSection.selectedField().date_format)
        : 0
        onActivated: fieldsModel.setProperty(
          editor.selectedFieldIndex,
          "date_format",
          logic.dateFormatValue(currentIndex)
        )
      }
    }

    RowLayout {
      Layout.fillWidth: true
      visible: logic ? logic.selectedFieldUsesText() : false
      spacing: Kirigami.Units.smallSpacing

      EditorFormField {
        label: "Prefix"
        Controls.TextField {
          Layout.fillWidth: true
          text: selectedSection.hasSelection() ? selectedSection.selectedField().prefix : ""
          placeholderText: LocaleText.tr("Optional prefix")
          onTextEdited: fieldsModel.setProperty(editor.selectedFieldIndex, "prefix", text)
        }
      }

      EditorFormField {
        label: "Suffix"
        Controls.TextField {
          Layout.fillWidth: true
          text: selectedSection.hasSelection() ? selectedSection.selectedField().suffix : ""
          placeholderText: LocaleText.tr("Optional suffix")
          onTextEdited: fieldsModel.setProperty(editor.selectedFieldIndex, "suffix", text)
        }
      }
    }

    EditorFormField {
      visible: logic ? logic.selectedFieldUsesText() : false
      label: "Font"
      FontFamilyPicker {
        Layout.fillWidth: true
        value: selectedSection.hasSelection() ? selectedSection.selectedField().font_family : "Noto Sans"
        onFamilySelected: family => selectedSection.applyToSelection("font_family", family)
      }
    }

    RowLayout {
      Layout.fillWidth: true
      visible: logic ? logic.selectedFieldUsesText() : false
      spacing: Kirigami.Units.smallSpacing

      EditorFormField {
        label: "Font size"
        Controls.SpinBox {
          Layout.fillWidth: true
          from: 5
          to: 200
          value: selectedSection.hasSelection()
          ? Math.round(selectedSection.selectedField().font_size_pt)
          : 12
          onValueModified: selectedSection.applyToSelection("font_size_pt", value)
        }
      }

      EditorFormField {
        label: "Style"
        Layout.fillWidth: false

        RowLayout {
          spacing: 0

          Controls.ToolButton {
            icon.name: AppIcons.pick("bold")
            text: LocaleText.tr("Bold")
            display: Controls.AbstractButton.IconOnly
            checkable: true
            checked: selectedSection.isBold()
            onToggled: selectedSection.applyToSelection("font_weight", checked ? 700 : 400)

            Controls.ToolTip.visible: hovered
            Controls.ToolTip.text: LocaleText.tr("Bold")
          }

          Controls.ToolButton {
            icon.name: AppIcons.pick("italic")
            text: LocaleText.tr("Italic")
            display: Controls.AbstractButton.IconOnly
            checkable: true
            checked: selectedSection.isItalic()
            onToggled: selectedSection.applyToSelection("font_italic", checked)

            Controls.ToolTip.visible: hovered
            Controls.ToolTip.text: LocaleText.tr("Italic")
          }
        }
      }
    }

    RowLayout {
      Layout.fillWidth: true
      visible: logic ? logic.selectedFieldUsesText() : false
      spacing: Kirigami.Units.smallSpacing

      EditorFormField {
        label: "Horizontal"
        Controls.ComboBox {
          Layout.fillWidth: true
          model: ["left", "center", "right"]
          currentIndex: selectedSection.hasSelection()
          ? selectedSection.alignmentIndex(selectedSection.selectedField().horizontal_alignment, ["left", "center", "right"])
          : 0
          onActivated: selectedSection.applyToSelection("horizontal_alignment", currentText)
        }
      }

      EditorFormField {
        label: "Vertical"
        Controls.ComboBox {
          Layout.fillWidth: true
          model: ["top", "center", "bottom"]
          currentIndex: selectedSection.hasSelection()
          ? selectedSection.alignmentIndex(selectedSection.selectedField().vertical_alignment, ["top", "center", "bottom"])
          : 0
          onActivated: selectedSection.applyToSelection("vertical_alignment", currentText)
        }
      }
    }

    DangerButton {
      Layout.fillWidth: true
      text: selectedSection.selectedCount() > 1
      ? LocaleText.tr("Delete selected fields")
      : LocaleText.tr("Delete selected field")
      onClicked: logic.removeSelectedField()
    }
  }
}
