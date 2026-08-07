import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"
import "."

ColumnLayout {
  id: boundsControls

  property var logic

  Layout.fillWidth: true
  spacing: Kirigami.Units.smallSpacing

  RowLayout {
    Layout.fillWidth: true
    spacing: Kirigami.Units.smallSpacing

    EditorFormField {
      label: "X"
      NumberInput {
        Layout.fillWidth: true
        realFrom: 0
        realTo: 9999
        realValue: boundsControls.logic ? boundsControls.logic.selectedFieldValue("x_mm", 0) : 0
        onValueModified: boundsControls.logic.setSelectedFieldNumber("x_mm", realValue)
      }
    }

    EditorFormField {
      label: "Y"
      NumberInput {
        Layout.fillWidth: true
        realFrom: 0
        realTo: 9999
        realValue: boundsControls.logic ? boundsControls.logic.selectedFieldValue("y_mm", 0) : 0
        onValueModified: boundsControls.logic.setSelectedFieldNumber("y_mm", realValue)
      }
    }
  }

  RowLayout {
    Layout.fillWidth: true
    spacing: Kirigami.Units.smallSpacing

    EditorFormField {
      label: "W"
      NumberInput {
        Layout.fillWidth: true
        realFrom: 1
        realTo: 9999
        realValue: boundsControls.logic ? boundsControls.logic.selectedFieldValue("width_mm", 1) : 1
        onValueModified: boundsControls.logic.setSelectedFieldNumber("width_mm", realValue)
      }
    }

    EditorFormField {
      label: "H"
      NumberInput {
        Layout.fillWidth: true
        realFrom: 1
        realTo: 9999
        realValue: boundsControls.logic ? boundsControls.logic.selectedFieldValue("height_mm", 1) : 1
        onValueModified: boundsControls.logic.setSelectedFieldNumber("height_mm", realValue)
      }
    }
  }

  RowLayout {
    Layout.fillWidth: true
    spacing: Kirigami.Units.smallSpacing

    Controls.Button {
      Layout.fillWidth: true
      text: LocaleText.tr("Center H")
      icon.name: "align-horizontal-center"
      onClicked: boundsControls.logic.centerSelectedFieldHorizontally()
    }
    Controls.Button {
      Layout.fillWidth: true
      text: LocaleText.tr("Center V")
      icon.name: "align-vertical-center"
      onClicked: boundsControls.logic.centerSelectedFieldVertically()
    }
    Controls.Button {
      Layout.fillWidth: true
      text: LocaleText.tr("Both")
      icon.name: "transform-move"
      onClicked: boundsControls.logic.centerSelectedFieldBoth()
    }
  }
}
