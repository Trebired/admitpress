import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

Item {
  id: controlsPane

  property var root
  property var printPage
  property int listMaxHeight: Kirigami.Units.gridUnit * 8
  readonly property int contentGap: Kirigami.Units.largeSpacing
  readonly property int actionGap: Kirigami.Units.largeSpacing

  Layout.fillHeight: true
  Layout.fillWidth: true
  Layout.preferredWidth: 760
  Layout.minimumWidth: 390

  Controls.ScrollView {
    id: controlsScroll

    anchors.fill: parent
    clip: true
    contentWidth: availableWidth
    contentHeight: controlsContentFrame.implicitHeight

    Controls.ScrollBar.horizontal.policy: Controls.ScrollBar.AlwaysOff

    Item {
      id: controlsContentFrame

      width: controlsScroll.availableWidth
      implicitHeight: controlsContent.implicitHeight + controlsPane.contentGap * 2

      ColumnLayout {
        id: controlsContent

        x: controlsPane.contentGap
        y: controlsPane.contentGap
        width: Math.max(0, controlsContentFrame.width - controlsPane.contentGap * 2)
        spacing: Kirigami.Units.smallSpacing

        Controls.Label {
          Layout.fillWidth: true
          text: LocaleText.tr("Tickets")
          font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
        }

        TicketNumberGrid {
          Layout.fillWidth: true
          model: printPage.quantityModel
          numberPadding: printPage.ticketNumberPadding()
          currentHeader: "Next"
          stepperHeader: "Quantity"
          maximumQuantity: 10000
          emptyText: "No prices"
          onRowActivated: function(index) {
            printPage.setActiveQuantityIndex(index)
          }

          onQuantityModified: function(index, nextQuantity) {
            printPage.setSelectionQuantity(index, nextQuantity)
          }
        }

        Item {
          Layout.fillWidth: true
          Layout.preferredHeight: controlsPane.actionGap - controlsContent.spacing
        }

        Controls.Button {
          Layout.fillWidth: true
          Layout.preferredHeight: 52
          text: LocaleText.tr("Print individually")
          icon.name: "printer"
          highlighted: true
          enabled: printPage.canSubmitSelection(false)
          font.pointSize: 14
          onClicked: printPage.submitSelection(false, false)
        }

        SectionBorder {}

        Controls.Label {
          Layout.fillWidth: true
          text: LocaleText.tr("Bulk ticket name")
        }

        Controls.TextField {
          Layout.fillWidth: true
          text: printPage.bulkName
          enabled: printPage.canEditBulkTicket()
          placeholderText: LocaleText.tr("Required for bulk ticket, optional for record")
          onTextEdited: {
            printPage.bulkName = text
            printPage.selectionRevision += 1
          }
        }

        Item {
          Layout.fillWidth: true
          Layout.preferredHeight: controlsPane.actionGap - controlsContent.spacing
        }

        Controls.Button {
          Layout.fillWidth: true
          Layout.preferredHeight: 52
          text: LocaleText.tr("Print bulk ticket")
          icon.name: "document-print-preview"
          enabled: printPage.canSubmitSelection(true)
          font.pointSize: 13
          onClicked: printPage.submitSelection(false, true)
        }

        SectionBorder {}

        Controls.Button {
          Layout.fillWidth: true
          Layout.preferredHeight: 52
          text: LocaleText.tr("Record without printing")
          icon.name: "view-financial-list"
          enabled: printPage.canSubmitSelection(false)
          font.pointSize: 13
          onClicked: printPage.submitSelection(true, false)
        }

        Controls.Label {
          Layout.fillWidth: true
          text: LocaleText.tr("Records the selected tickets without printing, for individual prints or bulk tickets.")
          wrapMode: Text.WordWrap
          color: Kirigami.Theme.disabledTextColor
        }

        SectionBorder {}

        ColumnLayout {
          Layout.fillWidth: true
          spacing: Kirigami.Units.smallSpacing

          Controls.Label {
            Layout.fillWidth: true
            text: LocaleText.tr("Today's prints")
            font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
          }

          PrintScrollList {
            id: recentList

            maxHeight: controlsPane.listMaxHeight
            model: printPage.summary && printPage.summary.date_print_operations
            ? printPage.summary.date_print_operations
            : []

            delegate: RecentPrintRow {
              required property var modelData
              printOperation: modelData
              onDetailRequested: function(printOperation) {
                root.printOperationDialogObject.openOperation(
                  printOperation,
                  printPage.summary ? printPage.summary.ticket : null
                )
              }
            }
          }
        }
      }
    }
  }
}
