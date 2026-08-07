import QtQuick.Controls as Controls
import org.kde.kirigami as Kirigami
import ".."

Controls.MenuItem {
  text: "Delete"
  icon.name: AppIcons.pick("delete")
  icon.color: Kirigami.Theme.negativeTextColor
  display: Controls.AbstractButton.TextBesideIcon
}
