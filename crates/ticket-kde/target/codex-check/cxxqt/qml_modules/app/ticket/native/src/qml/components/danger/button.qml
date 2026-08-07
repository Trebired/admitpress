import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Controls.Button {
  id: dangerButton

  property bool iconOnly: false
  property bool showTooltip: true

  text: LocaleText.tr("Delete")
  icon.name: "edit-delete"
  icon.color: Kirigami.Theme.negativeTextColor
  display: iconOnly ? Controls.AbstractButton.IconOnly : Controls.AbstractButton.TextBesideIcon
  Layout.preferredWidth: iconOnly ? Kirigami.Units.gridUnit * 2 : implicitWidth
  Layout.preferredHeight: iconOnly ? Kirigami.Units.gridUnit * 2 : implicitHeight

  Controls.ToolTip.visible: iconOnly && showTooltip && hovered
  Controls.ToolTip.text: text
}
