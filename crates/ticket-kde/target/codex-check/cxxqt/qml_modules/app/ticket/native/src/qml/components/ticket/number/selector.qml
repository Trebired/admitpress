import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../.."

ColumnLayout {
  id: ticketNumberGrid

  property var model: []
  property int numberPadding: 0
  property string priceHeader: "Price"
  property string currentHeader: "Current next"
  property string stepperHeader: "Quantity"
  property int minimumQuantity: 0
  property int maximumQuantity: 10000
  property int maxVisibleRows: 6
  property int stepperColumnWidth: 152 + Kirigami.Units.smallSpacing * 2
  property int rowHorizontalGap: Kirigami.Units.smallSpacing
  property int rowVerticalGap: Kirigami.Units.smallSpacing
  property int rowHeight: Math.max(56, Kirigami.Units.gridUnit * 3)
  property string emptyText: "No prices"
  readonly property int rowCount: model && model.count !== undefined ? model.count : 0
  readonly property int listMaxHeight: rowHeight * maxVisibleRows

  signal quantityModified(int index, int nextQuantity)
  signal rowActivated(int index)

  function preferredListHeight() {
    if (rowCount === 0) return 0
    return Math.min(rowsList.contentHeight || rowCount * rowHeight, listMaxHeight)
  }

  Layout.fillWidth: true
  spacing: Kirigami.Units.smallSpacing

  ListView {
    id: rowsList

    readonly property bool needsScrollbar: contentHeight > height

    Layout.fillWidth: true
    Layout.preferredHeight: ticketNumberGrid.preferredListHeight()
    Layout.maximumHeight: ticketNumberGrid.listMaxHeight
    visible: ticketNumberGrid.rowCount > 0
    clip: true
    boundsBehavior: Flickable.StopAtBounds
    interactive: contentHeight > height
    model: ticketNumberGrid.model

    Controls.ScrollBar.vertical: Controls.ScrollBar {
      id: rowsScrollBar

      policy: rowsList.needsScrollbar ? Controls.ScrollBar.AsNeeded : Controls.ScrollBar.AlwaysOff
    }

    Kirigami.Separator {
      x: 0
      y: 0
      width: parent.width
      visible: ticketNumberGrid.rowCount > 0
      z: 1
    }

    delegate: Item {
      id: ticketNumberRow

      required property int index
      required property string label
      required property int current_next
      required property int quantity

      width: ListView.view ? ListView.view.width : implicitWidth
      height: ticketNumberGrid.rowHeight

      RowLayout {
        x: ticketNumberGrid.rowHorizontalGap
        y: ticketNumberGrid.rowVerticalGap
        width: parent.width - ticketNumberGrid.rowHorizontalGap * 2
        height: parent.height - ticketNumberGrid.rowVerticalGap * 2
        spacing: Kirigami.Units.smallSpacing

        ColumnLayout {
          Layout.fillWidth: true
          Layout.alignment: Qt.AlignVCenter
          spacing: 0
          Controls.Label {
            Layout.fillWidth: true
            text: ticketNumberRow.label
            elide: Text.ElideRight
          }

          Controls.Label {
            Layout.fillWidth: true
            text: ticketNumberGrid.currentHeader
            + " "
            + TextFormat.displayTicketNumber(ticketNumberRow.current_next, ticketNumberGrid.numberPadding)
            elide: Text.ElideRight
            color: Kirigami.Theme.disabledTextColor
          }
        }

        QuantityStepper {
          Layout.preferredWidth: ticketNumberGrid.stepperColumnWidth
          Layout.alignment: Qt.AlignVCenter
          quantity: ticketNumberRow.quantity
          minimumQuantity: ticketNumberGrid.minimumQuantity
          maximumQuantity: ticketNumberGrid.maximumQuantity
          onQuantityModified: function(nextQuantity) {
            ticketNumberGrid.rowActivated(ticketNumberRow.index)
            ticketNumberGrid.quantityModified(ticketNumberRow.index, nextQuantity)
          }
        }
      }

      Kirigami.Separator {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
      }
    }
  }

  Kirigami.PlaceholderMessage {
    Layout.fillWidth: true
    visible: ticketNumberGrid.rowCount === 0
    text: ticketNumberGrid.emptyText
  }
}
