pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../../components"

GridLayout {
  id: fieldPalette

  property var root
  property var editor
  property var logic
  property var fieldPaletteModel

  Layout.fillWidth: true
  columns: 2
  columnSpacing: Kirigami.Units.smallSpacing
  rowSpacing: Kirigami.Units.smallSpacing

  Repeater {
    model: fieldPalette.fieldPaletteModel

    delegate: Item {
      id: paletteCell

      required property string label
      required property string fieldKind
      required property string dynamicKind
      required property string defaultText
      required property string iconRole

      readonly property bool alreadyAdded: fieldKind === "dynamic" && dynamicKind !== ""
      && !!fieldPalette.editor && fieldPalette.editor.fieldPaletteRevision >= 0
      && !!fieldPalette.logic && fieldPalette.logic.hasDynamicField(dynamicKind)
      readonly property bool usable: !alreadyAdded

      Layout.fillWidth: true
      Layout.preferredHeight: 42

      Rectangle {
        id: paletteItem

        anchors.fill: parent
        radius: 4
        color: !paletteCell.usable
        ? Kirigami.Theme.alternateBackgroundColor
        : paletteMouse.drag.active ? Kirigami.Theme.hoverColor : Kirigami.Theme.backgroundColor
        border.color: !paletteCell.usable ? Kirigami.Theme.disabledTextColor : Kirigami.Theme.textColor

        Controls.ToolTip.visible: paletteCell.alreadyAdded && hoverHandler.hovered
        Controls.ToolTip.text: LocaleText.tr("Already used in this ticket type")

        HoverHandler {
          id: hoverHandler
        }

        RowLayout {
          id: paletteItemContent

          readonly property int gap: Kirigami.Units.smallSpacing

          x: gap
          y: gap
          width: parent.width - gap * 2
          height: parent.height - gap * 2
          spacing: Kirigami.Units.smallSpacing

          Kirigami.Icon {
            Layout.preferredWidth: Kirigami.Units.iconSizes.smallMedium
            Layout.preferredHeight: Kirigami.Units.iconSizes.smallMedium
            source: AppIcons.pick(paletteCell.iconRole)
            opacity: paletteCell.usable ? 1 : 0.45
          }

          Controls.Label {
            Layout.fillWidth: true
            text: LocaleText.tr(paletteCell.label)
            elide: Text.ElideRight
            color: paletteCell.usable ? Kirigami.Theme.textColor : Kirigami.Theme.disabledTextColor
          }
        }
      }

      Item {
        id: dragProxy

        property string dragFieldKind: paletteCell.fieldKind
        property string dragDynamicKind: paletteCell.dynamicKind
        property string dragDefaultText: paletteCell.defaultText

        width: paletteCell.width
        height: paletteCell.height
        visible: paletteMouse.drag.active
        z: 20

        Drag.active: paletteMouse.drag.active
        Drag.keys: ["ticket-field"]
        Drag.supportedActions: Qt.CopyAction
        Drag.hotSpot.x: width / 2
        Drag.hotSpot.y: height / 2

        Rectangle {
          anchors.fill: parent
          radius: 4
          opacity: 0.85
          color: Kirigami.Theme.hoverColor
          border.color: Kirigami.Theme.textColor

          Controls.Label {
            x: Kirigami.Units.smallSpacing
            width: parent.width - Kirigami.Units.smallSpacing * 2
            anchors.verticalCenter: parent.verticalCenter
            horizontalAlignment: Text.AlignHCenter
            text: LocaleText.tr(paletteCell.label)
            elide: Text.ElideRight
          }
        }
      }

      MouseArea {
        id: paletteMouse

        anchors.fill: parent
        enabled: paletteCell.usable
        drag.target: dragProxy
        onReleased: {
          dragProxy.Drag.drop()
          dragProxy.x = 0
          dragProxy.y = 0
        }
        onClicked: fieldPalette.logic.addField(
          paletteCell.fieldKind,
          paletteCell.dynamicKind,
          paletteCell.defaultText
        )
      }
    }
  }
}
