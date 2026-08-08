import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: reorderDialog

  property var backend: root ? root.backendObject : null

  title: LocaleText.tr("Reorder tickets")
  dialogWidth: 420

  ListModel { id: orderModel }

  function openWithCurrentTickets() {
    orderModel.clear()
    const tickets = root.appState.tickets || []
    for (const summary of tickets) {
      orderModel.append({ ticketId: summary.ticket.id, name: summary.ticket.name })
    }
    open()
  }

  function submit() {
    const order = []
    for (let index = 0; index < orderModel.count; ++index) {
      order.push(orderModel.get(index).ticketId)
    }
    const data = root.acceptResponse(
      backend.setHomeTicketOrder(JSON.stringify(order)),
      LocaleText.tr("Ticket order updated")
    )
    if (data) reorderDialog.close()
  }

  contentItem: ColumnLayout {
    spacing: reorderDialog.contentSpacing

    Controls.Label {
      Layout.fillWidth: true
      wrapMode: Text.WordWrap
      color: Kirigami.Theme.disabledTextColor
      text: LocaleText.tr("Drag tickets into the order you want them to appear in.")
    }

    ListView {
      id: orderView

      Layout.fillWidth: true
      Layout.preferredHeight: Math.min(contentHeight, Kirigami.Units.gridUnit * 16)
      clip: true
      spacing: Kirigami.Units.smallSpacing
      model: orderModel

      delegate: Item {
        id: orderRow

        required property int index
        required property string name

        width: orderView.width
        height: rowContent.implicitHeight

        RowLayout {
          id: rowContent

          width: orderRow.width
          spacing: Kirigami.Units.smallSpacing

          Drag.active: dragHandle.drag.active
          Drag.source: orderRow
          Drag.hotSpot.y: height / 2

          states: State {
            when: dragHandle.drag.active
            ParentChange { target: rowContent; parent: orderView }
          }

          Kirigami.Icon {
            source: "transform-move"
            opacity: 0.6
            Layout.preferredWidth: Kirigami.Units.iconSizes.small
            Layout.preferredHeight: Kirigami.Units.iconSizes.small

            MouseArea {
              id: dragHandle

              anchors.fill: parent
              cursorShape: Qt.SizeVerCursor
              drag.target: rowContent
              drag.axis: Drag.YAxis
              onReleased: rowContent.Drag.drop()
            }
          }

          Controls.Label {
            Layout.fillWidth: true
            text: orderRow.name
            elide: Text.ElideRight
          }
        }

        DropArea {
          anchors.fill: parent
          onEntered: (drag) => DragReorder.moveOnDrop(orderModel, drag, orderRow, orderRow.index)
        }
      }
    }

    DialogActions {
      Controls.Button {
        text: LocaleText.tr("Save")
        icon.name: AppIcons.pick("ok")
        highlighted: true
        onClicked: reorderDialog.submit()
      }
    }
  }
}
