import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

PrintListRow {
  id: salesRow

  property bool showDisclosure: false
  property bool collapsed: false
  property bool selectionEnabled: true
  property bool selectionTristate: false
  property int selectionState: Qt.Unchecked
  property string title: ""
  property bool titleBold: false
  property bool titleMuted: false
  property string leadingStat: ""
  property string ticketsText: ""
  property string moneyText: ""
  property string detailsTooltip: ""

  signal selectionToggled
  signal disclosureToggled
  signal detailsRequested

  hoverEnabled: salesRow.showDisclosure
  onClicked: if (salesRow.showDisclosure) salesRow.disclosureToggled()

  Controls.CheckBox {
    Layout.preferredWidth: Kirigami.Units.gridUnit * 1.5
    Layout.alignment: Qt.AlignVCenter
    padding: 0
    enabled: salesRow.selectionEnabled
    tristate: salesRow.selectionTristate
    checkState: salesRow.selectionState
    onClicked: salesRow.selectionToggled()
  }

  Kirigami.Icon {
    Layout.preferredWidth: Kirigami.Units.iconSizes.small
    Layout.preferredHeight: Kirigami.Units.iconSizes.small
    Layout.alignment: Qt.AlignVCenter
    visible: salesRow.showDisclosure
    source: salesRow.collapsed ? "arrow-right" : "arrow-down"
  }

  Controls.Label {
    Layout.fillWidth: true
    Layout.minimumWidth: Kirigami.Units.gridUnit * 5
    Layout.alignment: Qt.AlignVCenter
    text: salesRow.title
    font.bold: salesRow.titleBold
    font.pointSize: salesRow.titleBold
    ? Kirigami.Theme.defaultFont.pointSize + 1
    : Kirigami.Theme.defaultFont.pointSize
    elide: Text.ElideRight
    color: salesRow.titleMuted ? Kirigami.Theme.disabledTextColor : Kirigami.Theme.textColor
  }

  Controls.Label {
    Layout.preferredWidth: Kirigami.Units.gridUnit * 4
    Layout.alignment: Qt.AlignVCenter
    text: salesRow.leadingStat
    horizontalAlignment: Text.AlignRight
    elide: Text.ElideRight
  }

  Controls.Label {
    Layout.preferredWidth: Kirigami.Units.gridUnit * 4.8
    Layout.alignment: Qt.AlignVCenter
    text: salesRow.ticketsText
    horizontalAlignment: Text.AlignRight
    elide: Text.ElideRight
  }

  Controls.Label {
    Layout.preferredWidth: Kirigami.Units.gridUnit * 5
    Layout.alignment: Qt.AlignVCenter
    text: salesRow.moneyText
    horizontalAlignment: Text.AlignRight
    elide: Text.ElideRight
  }

  Controls.ToolButton {
    Layout.preferredWidth: Kirigami.Units.gridUnit * 2
    Layout.alignment: Qt.AlignVCenter
    display: Controls.AbstractButton.IconOnly
    icon.name: AppIcons.pick("details")
    text: salesRow.detailsTooltip
    onClicked: salesRow.detailsRequested()
  }
}
