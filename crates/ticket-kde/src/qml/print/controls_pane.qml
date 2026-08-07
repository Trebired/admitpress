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

  ColumnLayout {
    anchors.fill: parent
    spacing: 0

    Item {
      id: printControlsTopBar

      property int gap: Kirigami.Units.smallSpacing

      Layout.fillWidth: true
      implicitHeight: printControlsTopRow.implicitHeight + gap * 2 + 1

      RowLayout {
        id: printControlsTopRow

        x: printControlsTopBar.gap
        y: printControlsTopBar.gap
        width: printControlsTopBar.width - printControlsTopBar.gap * 2
        height: implicitHeight
        spacing: printControlsTopBar.gap

        DateSelector {
          selectedDate: printPage.localDate
          disabledDates: printPage.lockedDateMap
          onDateSelected: function(localDate) {
            printPage.switchToDate(localDate)
          }
        }

        Item { Layout.fillWidth: true }

        Controls.Label {
          text: printPage.selectionSummaryText()
          font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
        }

        Controls.ToolButton {
          text: LocaleText.tr("Counters")
          icon.name: AppIcons.pick("overflow")
          enabled: printPage.summary !== null
          onClicked: counterMenu.popup()

          Controls.Menu {
            id: counterMenu

            Controls.MenuItem {
              text: LocaleText.tr("Adjust counters")
              icon.name: AppIcons.pick("number")
              enabled: printPage.summary !== null && !printPage.isCurrentDayLocked()
              onTriggered: printPage.openBulkCounterDialog()
            }

            DangerMenuItem {
              text: LocaleText.tr("Revert last sale")
              icon.name: AppIcons.pick("undo")
              enabled: printPage.canRevertLastSale()
              onTriggered: printPage.openRevertLastSaleDialog()
            }
          }
        }
      }

      Kirigami.Separator {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
      }
    }

    Kirigami.InlineMessage {
      Layout.fillWidth: true
      visible: printPage.isCurrentDayLocked()
      type: Kirigami.MessageType.Warning
      text: LocaleText.tr("This day is locked because it already has sales and is not today.")
    }

    Controls.ScrollView {
      id: controlsScroll

      Layout.fillWidth: true
      Layout.fillHeight: true
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

          RowLayout {
            Layout.fillWidth: true
            spacing: Kirigami.Units.smallSpacing

            Controls.Label {
              Layout.fillWidth: true
              text: LocaleText.tr("Tickets")
              font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
            }

            CurrencyPicker {
              Layout.preferredWidth: Kirigami.Units.gridUnit * 12
              Layout.fillWidth: false
              root: controlsPane.root
              value: printPage.saleCurrency || (controlsPane.root.appState.main_currency || "")
              placeholderLabel: LocaleText.tr("Currency")
              onCurrencySelected: function(currency) {
                printPage.setSaleCurrency(
                  currency === (controlsPane.root.appState.main_currency || "") ? "" : currency
                )
              }
            }

            DangerButton {
              iconOnly: true
              flat: true
              text: LocaleText.tr("Reset selection")
              icon.name: AppIcons.pick("reset")
              enabled: printPage.selectedCount() > 0
              onClicked: printPage.resetSelection()
            }

            Controls.ToolButton {
              text: LocaleText.tr("Edit prices")
              icon.name: AppIcons.pick("edit")
              enabled: printPage.summary !== null
              onClicked: root.pricesDialogObject.openFor(
                printPage.ticketId,
                printPage.localDate,
                printPage.summary ? printPage.summary.subtickets : []
              )
            }
          }

          TicketNumberGrid {
            Layout.fillWidth: true
            model: printPage.quantityModel
            numberPadding: printPage.ticketNumberPadding()
            currentHeader: "Next"
            stepperHeader: "Quantity"
            maximumQuantity: 10000
            quantityEnabled: !printPage.isCurrentDayLocked()
            emptyText: LocaleText.tr("No prices yet")
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

          Controls.Label {
            Layout.fillWidth: true
            text: LocaleText.tr("Sale name")
          }

          Controls.TextField {
            Layout.fillWidth: true
            text: printPage.saleName
            enabled: printPage.canNameSale()
            placeholderText: LocaleText.tr("Name this sale (optional)")
            onTextEdited: {
              printPage.saleName = text
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
            text: LocaleText.tr("Print tickets")
            icon.name: AppIcons.pick("printer")
            highlighted: true
            enabled: printPage.canSubmitSelection(false)
            font.pointSize: 14
            onClicked: printPage.submitSelection(false, false)
          }

          RowLayout {
            Layout.fillWidth: true
            spacing: Kirigami.Units.smallSpacing

            Controls.Button {
              Layout.fillWidth: true
              Layout.preferredWidth: 0
              Layout.preferredHeight: 52
              text: LocaleText.tr("Print group ticket")
              icon.name: AppIcons.pick("printPreview")
              enabled: printPage.canSubmitSelection(true)
              font.pointSize: 13
              onClicked: printPage.submitSelection(false, true)
            }

            Controls.Button {
              Layout.fillWidth: true
              Layout.preferredWidth: 0
              Layout.preferredHeight: 52
              text: LocaleText.tr("Record only")
              icon.name: AppIcons.pick("recorded")
              enabled: printPage.canSubmitSelection(false)
              font.pointSize: 13
              onClicked: printPage.submitSelection(true, false)
            }
          }

          SectionBorder {}

          ColumnLayout {
            Layout.fillWidth: true
            spacing: Kirigami.Units.smallSpacing

            Controls.Label {
              Layout.fillWidth: true
              text: LocaleText.tr("Today's activity")
              font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
            }

            RowLayout {
              Layout.fillWidth: true
              spacing: Kirigami.Units.smallSpacing
              visible: printPage.hasActivity()

              Controls.TextField {
                Layout.fillWidth: true
                text: printPage.activitySearch
                placeholderText: LocaleText.tr("Search activity")
                onTextEdited: printPage.activitySearch = text
              }

              Controls.ComboBox {
                id: activityFilterCombo

                Layout.preferredWidth: Kirigami.Units.gridUnit * 9
                model: printPage.activityFilterChoices()
                textRole: "text"
                valueRole: "kind"
                currentIndex: {
                  const choices = printPage.activityFilterChoices()
                  const index = choices.findIndex(function(choice) {
                      return choice.kind === printPage.activityFilter
                  })
                  return index >= 0 ? index : 0
                }
                onActivated: function(index) {
                  printPage.activityFilter = activityFilterCombo.valueAt(index)
                }
              }
            }

            PrintScrollList {
              id: activityList

              maxHeight: controlsPane.listMaxHeight
              model: printPage.activityEntries()

              delegate: ActivityRow {
                required property var modelData
                entry: modelData
                onDetailRequested: function(operation) {
                  root.printOperationDialogObject.openOperation(
                    operation,
                    printPage.summary ? printPage.summary.ticket : null
                  )
                }
              }
            }

            EmptyState {
              visible: !printPage.hasActivity()
              text: LocaleText.tr("Nothing has been sold or recorded today yet.")
            }

            EmptyState {
              visible: printPage.hasActivity() && printPage.activityEntries().length === 0
              text: LocaleText.tr("Nothing matches this filter")
            }
          }
        }
      }
    }
  }
}
