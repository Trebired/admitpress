import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: pricesDialog

  property var backend: root ? root.backendObject : null
  property string ticketId: ""
  property string localDate: ""

  title: LocaleText.tr("Edit prices")
  dialogWidth: 460

  ListModel {
    id: pricesModel
  }

  function openFor(ticketIdValue, localDateValue, subtickets) {
    ticketId = String(ticketIdValue || "")
    localDate = String(localDateValue || "")
    pricesModel.clear()
    const mainCurrency = root.appState.main_currency || ""
    for (const subticket of (subtickets || [])) {
      const row = Object.assign({}, subticket, { currency: subticket.currency || mainCurrency })
      pricesModel.append(TextFormat.normalizeSubticketRow(row))
    }
    open()
  }

  function submit() {
    const subticketsJson = JSON.stringify(TextFormat.serializeSubticketModel(pricesModel))
    const data = root.acceptResponse(
      backend.updateSubtickets(pricesDialog.ticketId, pricesDialog.localDate, subticketsJson),
      LocaleText.tr("Prices updated")
    )
    if (data && root.pageStack.currentItem && root.pageStack.currentItem.load) {
      root.pageStack.currentItem.load()
    }
    if (data) pricesDialog.close()
  }

  contentItem: ColumnLayout {
    spacing: pricesDialog.contentSpacing

    Controls.Label {
      Layout.fillWidth: true
      wrapMode: Text.WordWrap
      color: Kirigami.Theme.disabledTextColor
      text: root.appState.main_currency
      ? LocaleText.tr("All prices are in ") + root.appState.main_currency
      : LocaleText.tr("Choose a main currency in Settings first.")
    }

    ListView {
      id: pricesView

      Layout.fillWidth: true
      Layout.preferredHeight: Math.min(contentHeight, Kirigami.Units.gridUnit * 16)
      clip: true
      spacing: Kirigami.Units.smallSpacing
      model: pricesModel

      delegate: Item {
        id: priceRow

        required property int index
        required property int amount_minor

        width: pricesView.width
        height: rowContent.implicitHeight

        RowLayout {
          id: rowContent

          width: priceRow.width
          spacing: Kirigami.Units.smallSpacing

          Drag.active: dragHandle.drag.active
          Drag.source: priceRow
          Drag.hotSpot.y: height / 2

          states: State {
            when: dragHandle.drag.active
            ParentChange { target: rowContent; parent: pricesView }
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
            text: LocaleText.tr("Price") + " " + (priceRow.index + 1)
          }

          NumberInput {
            Layout.fillWidth: true
            decimals: 2
            realFrom: 0
            realTo: 999999
            realStepSize: 1
            realValue: Number(priceRow.amount_minor || 0) / 100
            onValueModified: pricesModel.setProperty(priceRow.index, "amount_minor", Math.round(realValue * 100))
          }

          DangerButton {
            iconOnly: true
            showTooltip: false
            text: LocaleText.tr("Remove price")
            onClicked: pricesModel.remove(priceRow.index)
          }
        }

        DropArea {
          anchors.fill: parent
          onEntered: (drag) => {
            if (drag.source && drag.source !== priceRow) {
              pricesModel.move(drag.source.index, priceRow.index, 1)
            }
          }
        }
      }
    }

    Controls.Button {
      Layout.fillWidth: true
      text: LocaleText.tr("Add price")
      icon.name: AppIcons.pick("add")
      onClicked: pricesModel.append(
        TextFormat.normalizeSubticketRow({ currency: root.appState.main_currency || "" })
      )
    }

    DialogActions {
      Controls.Button {
        text: LocaleText.tr("Save")
        icon.name: AppIcons.pick("ok")
        highlighted: true
        onClicked: pricesDialog.submit()
      }
    }
  }
}
