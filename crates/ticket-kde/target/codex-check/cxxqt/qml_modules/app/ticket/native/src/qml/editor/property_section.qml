import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

ColumnLayout {
  id: propertySection

  default property alias sectionContent: sectionBody.data
  property string title: ""
  property string subtitle: ""

  Layout.fillWidth: true
  spacing: Kirigami.Units.smallSpacing

  Controls.Label {
    Layout.fillWidth: true
    visible: propertySection.title !== ""
    text: LocaleText.tr(propertySection.title)
    elide: Text.ElideRight
    font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
  }

  Controls.Label {
    Layout.fillWidth: true
    visible: propertySection.subtitle !== ""
    text: LocaleText.tr(propertySection.subtitle)
    wrapMode: Text.WordWrap
    color: Kirigami.Theme.disabledTextColor
  }

  ColumnLayout {
    id: sectionBody

    Layout.fillWidth: true
    spacing: Kirigami.Units.smallSpacing
  }
}
