import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

PrintListRow {
  id: activityRow

  required property var entry
  signal detailRequested(var operation)

  readonly property bool openable: entry.kind === "print"

  onClicked: {
    if (activityRow.openable) activityRow.detailRequested(activityRow.entry.operation)
  }

  Kirigami.Icon {
    Layout.preferredWidth: Kirigami.Units.iconSizes.smallMedium
    Layout.preferredHeight: Kirigami.Units.iconSizes.smallMedium
    Layout.alignment: Qt.AlignVCenter
    source: AppIcons.pick(activityRow.entry.iconRole)
  }

  TicketNumberLabel {
    Layout.alignment: Qt.AlignVCenter
    value: activityRow.entry.number
  }

  ColumnLayout {
    Layout.fillWidth: true
    Layout.alignment: Qt.AlignVCenter
    spacing: 0

    Controls.Label {
      Layout.fillWidth: true
      text: activityRow.entry.title
      elide: Text.ElideRight
    }

    Controls.Label {
      Layout.fillWidth: true
      text: activityRow.entry.subtitle
      elide: Text.ElideRight
      font.pointSize: Kirigami.Theme.smallFont.pointSize
      color: Kirigami.Theme.disabledTextColor
    }
  }
}
