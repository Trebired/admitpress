import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

ColumnLayout {
  id: formField

  default property alias fieldContent: fieldBody.data
  property string label: ""

  Layout.fillWidth: true
  spacing: Kirigami.Units.smallSpacing

  Controls.Label {
    Layout.fillWidth: true
    visible: formField.label !== ""
    text: LocaleText.tr(formField.label)
    elide: Text.ElideRight
    color: Kirigami.Theme.disabledTextColor
  }

  ColumnLayout {
    id: fieldBody

    Layout.fillWidth: true
    spacing: Kirigami.Units.smallSpacing
  }
}
