import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import QtQuick.Window
import org.kde.kirigami as Kirigami

ColumnLayout {
  id: salesTable

  property var rows: []
  property color lineColor: Qt.rgba(
    Kirigami.Theme.textColor.r,
    Kirigami.Theme.textColor.g,
    Kirigami.Theme.textColor.b,
    0.22
  )
  readonly property real lineWidth: Screen.devicePixelRatio > 0 ? 1 / Screen.devicePixelRatio : 1

  Layout.fillWidth: true
  spacing: 0

  function amountText(row) {
    return row ? MoneyFormat.format(row.amount_minor, row.currency) : ""
  }

  function totalText(row) {
    return row ? MoneyFormat.format(row.total_minor, row.currency) : ""
  }

  Item {
    id: tableFrame

    Layout.fillWidth: true
    implicitHeight: tableContent.implicitHeight + salesTable.lineWidth * 2

    ColumnLayout {
      id: tableContent

      x: salesTable.lineWidth
      y: salesTable.lineWidth
      width: parent.width - salesTable.lineWidth * 2
      height: parent.height - salesTable.lineWidth * 2
      spacing: 0

      Rectangle {
        id: headerRow

        readonly property int gap: Kirigami.Units.smallSpacing

        Layout.fillWidth: true
        implicitHeight: headerGrid.implicitHeight + gap * 2
        color: Kirigami.Theme.alternateBackgroundColor

        GridLayout {
          id: headerGrid

          x: headerRow.gap
          y: headerRow.gap
          width: parent.width - headerRow.gap * 2
          height: parent.height - headerRow.gap * 2
          columns: 5
          columnSpacing: Kirigami.Units.smallSpacing
          rowSpacing: 0

          Controls.Label {
            Layout.fillWidth: true
            text: LocaleText.tr("Price")
          }
          Rectangle {
            Layout.minimumWidth: salesTable.lineWidth
            Layout.preferredWidth: salesTable.lineWidth
            Layout.maximumWidth: salesTable.lineWidth
            Layout.fillHeight: true
            color: salesTable.lineColor
          }
          Controls.Label {
            Layout.preferredWidth: 90
            text: LocaleText.tr("Quantity")
            horizontalAlignment: Text.AlignRight
          }
          Rectangle {
            Layout.minimumWidth: salesTable.lineWidth
            Layout.preferredWidth: salesTable.lineWidth
            Layout.maximumWidth: salesTable.lineWidth
            Layout.fillHeight: true
            color: salesTable.lineColor
          }
          Controls.Label {
            Layout.preferredWidth: 120
            text: LocaleText.tr("Total")
            horizontalAlignment: Text.AlignRight
          }
        }
      }

      Rectangle {
        Layout.fillWidth: true
        Layout.minimumHeight: salesTable.rows.length > 0 ? salesTable.lineWidth : 0
        Layout.preferredHeight: salesTable.rows.length > 0 ? salesTable.lineWidth : 0
        Layout.maximumHeight: salesTable.rows.length > 0 ? salesTable.lineWidth : 0
        color: salesTable.lineColor
        visible: salesTable.rows.length > 0
      }

      Repeater {
        model: salesTable.rows || []

        delegate: ColumnLayout {
          required property int index
          required property var modelData

          Layout.fillWidth: true
          spacing: 0

          Rectangle {
            id: salesRow

            readonly property int gap: Kirigami.Units.smallSpacing

            Layout.fillWidth: true
            implicitHeight: rowGrid.implicitHeight + gap * 2
            color: Kirigami.Theme.backgroundColor

            GridLayout {
              id: rowGrid

              x: salesRow.gap
              y: salesRow.gap
              width: parent.width - salesRow.gap * 2
              height: parent.height - salesRow.gap * 2
              columns: 5
              columnSpacing: Kirigami.Units.smallSpacing
              rowSpacing: 0

              Controls.Label {
                Layout.fillWidth: true
                text: salesTable.amountText(modelData)
                elide: Text.ElideRight
              }
              Rectangle {
                Layout.minimumWidth: salesTable.lineWidth
                Layout.preferredWidth: salesTable.lineWidth
                Layout.maximumWidth: salesTable.lineWidth
                Layout.fillHeight: true
                color: salesTable.lineColor
              }
              Controls.Label {
                Layout.preferredWidth: 90
                text: modelData.quantity + "x"
                horizontalAlignment: Text.AlignRight
              }
              Rectangle {
                Layout.minimumWidth: salesTable.lineWidth
                Layout.preferredWidth: salesTable.lineWidth
                Layout.maximumWidth: salesTable.lineWidth
                Layout.fillHeight: true
                color: salesTable.lineColor
              }
              Controls.Label {
                Layout.preferredWidth: 120
                text: salesTable.totalText(modelData)
                horizontalAlignment: Text.AlignRight
              }
            }
          }

          Rectangle {
            Layout.fillWidth: true
            Layout.minimumHeight: index < salesTable.rows.length - 1 ? salesTable.lineWidth : 0
            Layout.preferredHeight: index < salesTable.rows.length - 1 ? salesTable.lineWidth : 0
            Layout.maximumHeight: index < salesTable.rows.length - 1 ? salesTable.lineWidth : 0
            color: salesTable.lineColor
            visible: index < salesTable.rows.length - 1
          }
        }
      }
    }

    Rectangle {
      anchors.left: parent.left
      anchors.right: parent.right
      anchors.top: parent.top
      height: salesTable.lineWidth
      color: salesTable.lineColor
    }
    Rectangle {
      anchors.left: parent.left
      anchors.right: parent.right
      anchors.bottom: parent.bottom
      height: salesTable.lineWidth
      color: salesTable.lineColor
    }
    Rectangle {
      anchors.left: parent.left
      anchors.top: parent.top
      anchors.bottom: parent.bottom
      width: salesTable.lineWidth
      color: salesTable.lineColor
    }
    Rectangle {
      anchors.right: parent.right
      anchors.top: parent.top
      anchors.bottom: parent.bottom
      width: salesTable.lineWidth
      color: salesTable.lineColor
    }
  }
}
