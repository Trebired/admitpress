import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"
import "."

Kirigami.ScrollablePage {
  id: propertiesPane

  property var root
  property var editor
  property var logic
  property var fieldsModel
  property var fieldPaletteModel

  property alias ticketName: ticketSection.ticketName
  property alias widthValue: ticketSection.widthValue
  property alias heightValue: ticketSection.heightValue
  property alias dpiValue: ticketSection.dpiValue
  property alias borderWidthValue: ticketSection.borderWidthValue
  property alias startValue: numberingSection.startValue
  property alias incrementValue: numberingSection.incrementValue
  property alias paddingValue: numberingSection.paddingValue
  property alias numberPrefix: numberingSection.numberPrefix
  property alias numberSuffix: numberingSection.numberSuffix

  title: LocaleText.tr("Properties")
  padding: Kirigami.Units.largeSpacing

  ColumnLayout {
    width: propertiesPane.availableWidth

    EditorSelectedFieldSection {
      id: selectedFieldSection
      Layout.fillWidth: true
      visible: selectedFieldSection.hasSelection()
      root: propertiesPane.root
      editor: propertiesPane.editor
      logic: propertiesPane.logic
      fieldsModel: propertiesPane.fieldsModel
    }

    EditorNumberingSection {
      id: numberingSection
      visible: !!propertiesPane.logic && propertiesPane.logic.selectedFieldIsNumber()
    }

    SectionBorder {
      visible: selectedFieldSection.visible
    }

    EditorPropertySection {
      title: "Fields"

      Controls.CheckBox {
        Layout.fillWidth: true
        text: LocaleText.tr("Snap to centered grid")
        checked: editor ? editor.snapToGrid : true
        onToggled: editor.snapToGrid = checked
      }

      Controls.Button {
        Layout.fillWidth: true
        text: LocaleText.tr("Add image")
        icon.name: AppIcons.pick("image")
        onClicked: root.imageDialogObject.openFor(editor.ticketId)
      }

      EditorFieldPalette {
        root: propertiesPane.root
        editor: propertiesPane.editor
        logic: propertiesPane.logic
        fieldPaletteModel: propertiesPane.fieldPaletteModel
      }
    }

    SectionBorder {}

    EditorTicketSection {
      id: ticketSection
      root: propertiesPane.root
      editor: propertiesPane.editor
      logic: propertiesPane.logic
    }

    Item { Layout.fillHeight: true }
  }
}
