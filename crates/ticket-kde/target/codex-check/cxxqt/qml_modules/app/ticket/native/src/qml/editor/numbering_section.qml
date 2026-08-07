import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"
import "."

EditorPropertySection {
  id: numbering

  property alias startValue: startField.value
  property alias incrementValue: incrementField.value
  property alias paddingValue: paddingField.value
  property alias numberPrefix: prefixField.text
  property alias numberSuffix: suffixField.text

  title: "Numbering"

  RowLayout {
    Layout.fillWidth: true
    spacing: Kirigami.Units.smallSpacing
    EditorFormField {
      label: "Start"
      Controls.SpinBox {
        id: startField
        Layout.fillWidth: true
        from: 1
        to: 999999999
        value: 1
        editable: true
      }
    }
    EditorFormField {
      label: "Step"
      Controls.SpinBox {
        id: incrementField
        Layout.fillWidth: true
        from: 1
        to: 1000
        value: 1
        editable: true
      }
    }
  }

  EditorFormField {
    label: "Minimum digits"
    Controls.SpinBox {
      id: paddingField
      Layout.fillWidth: true
      from: 0
      to: 12
      value: 0
    }
  }

  EditorFormField {
    label: "Prefix"
    Controls.TextField {
      id: prefixField
      Layout.fillWidth: true
      placeholderText: LocaleText.tr("Optional prefix")
    }
  }

  EditorFormField {
    label: "Suffix"
    Controls.TextField {
      id: suffixField
      Layout.fillWidth: true
      placeholderText: LocaleText.tr("Optional suffix")
    }
  }
}
