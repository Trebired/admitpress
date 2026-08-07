import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Controls.Label {
  id: emptyState

  Layout.fillWidth: true
  Layout.topMargin: Kirigami.Units.largeSpacing
  Layout.bottomMargin: Kirigami.Units.largeSpacing
  horizontalAlignment: Text.AlignHCenter
  wrapMode: Text.WordWrap
  color: Kirigami.Theme.disabledTextColor
}
