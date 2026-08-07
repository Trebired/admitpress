import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

Item {
  id: daysPane

  property var root
  property var printPage
  readonly property int contentGap: Kirigami.Units.largeSpacing

  Layout.fillHeight: true
  Layout.fillWidth: true
  Layout.preferredWidth: 760
  Layout.minimumWidth: 390

  Controls.ScrollView {
    id: daysScroll

    anchors.fill: parent
    clip: true
    contentWidth: availableWidth
    contentHeight: daysContentFrame.implicitHeight

    Controls.ScrollBar.horizontal.policy: Controls.ScrollBar.AlwaysOff

    Item {
      id: daysContentFrame

      width: daysScroll.availableWidth
      implicitHeight: daysContent.implicitHeight + daysPane.contentGap * 2

      ColumnLayout {
        id: daysContent

        x: daysPane.contentGap
        y: daysPane.contentGap
        width: Math.max(0, daysContentFrame.width - daysPane.contentGap * 2)
        spacing: Kirigami.Units.smallSpacing

        RowLayout {
          id: daysHeader

          Layout.fillWidth: true

          Controls.Label {
            text: LocaleText.tr("Sales")
            font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
          }

          Controls.Label {
            Layout.fillWidth: true
            text: printPage.selectedDaySummaryText()
            horizontalAlignment: Text.AlignRight
            elide: Text.ElideRight
            color: printPage.selectedDayCount() > 0
            ? Kirigami.Theme.textColor
            : Kirigami.Theme.disabledTextColor
          }

          Controls.CheckBox {
            text: LocaleText.tr("Show empty")
            checked: printPage.showEmptyDays
            padding: 0
            onClicked: printPage.setShowEmptyDays(checked)
          }

          Controls.CheckBox {
            text: LocaleText.tr("All")
            enabled: printPage.hasSelectableDays()
            padding: 0
            tristate: true
            checkState: printPage.allSelectionState()
            onClicked: {
              if (printPage.allSelectionState() === Qt.Checked) {
                printPage.clearSelectedDays()
              } else {
                printPage.selectAllDays()
              }
            }
          }

          Controls.ToolButton {
            text: LocaleText.tr("Export")
            icon.name: AppIcons.pick("export")
            enabled: printPage.selectedDayCount() > 0
            onClicked: exportMenu.popup()

            Controls.Menu {
              id: exportMenu

              Controls.MenuItem {
                text: LocaleText.tr("Excel workbook") + " (*.xlsx)"
                onTriggered: printPage.exportSelectedDays("xlsx")
              }

              Controls.MenuItem {
                text: LocaleText.tr("OpenDocument spreadsheet") + " (*.ods)"
                onTriggered: printPage.exportSelectedDays("ods")
              }
            }
          }
        }

        Repeater {
          model: printPage.visibleMonthGroups()

          delegate: ColumnLayout {
            id: monthGroup

            required property var modelData
            readonly property bool collapsed: printPage.isMonthCollapsed(modelData.key)

            Layout.fillWidth: true
            spacing: 0

            SalesRow {
              Layout.fillWidth: true
              showDisclosure: true
              collapsed: monthGroup.collapsed
              titleBold: true
              title: printPage.monthLabel(monthGroup.modelData.key)
              leadingStat: TextFormat.dayCount(monthGroup.modelData.dayCount)
              ticketsText: TextFormat.ticketCount(monthGroup.modelData.ticketTotal)
              moneyText: MoneyFormat.format(
                monthGroup.modelData.moneyTotal,
                monthGroup.modelData.currency
              )
              detailsTooltip: LocaleText.tr("Month details")
              selectionTristate: true
              selectionState: printPage.monthSelectionState(monthGroup.modelData.key)
              onSelectionToggled: printPage.setMonthSelected(
                monthGroup.modelData.key,
                printPage.monthSelectionState(monthGroup.modelData.key) !== Qt.Checked
              )
              onDisclosureToggled: printPage.toggleMonthCollapsed(monthGroup.modelData.key)
              onDetailsRequested: printPage.openMonthDetailDialog(monthGroup.modelData.key)
            }

            ColumnLayout {
              Layout.fillWidth: true
              Layout.leftMargin: Kirigami.Units.gridUnit
              spacing: 0
              visible: !monthGroup.collapsed

              Repeater {
                model: monthGroup.modelData.days

                delegate: SalesRow {
                  id: dayRow

                  required property var modelData
                  readonly property bool hasSales: Number(modelData.ticket_count || 0) > 0

                  Layout.fillWidth: true
                  title: TextFormat.dayDate(modelData.local_date)
                  titleMuted: !dayRow.hasSales
                  ticketsText: TextFormat.ticketCount(modelData.ticket_count)
                  moneyText: MoneyFormat.format(modelData.total_minor, modelData.currency)
                  detailsTooltip: LocaleText.tr("Day details")
                  selectionEnabled: dayRow.hasSales
                  selectionState: printPage.daySelected(modelData.local_date)
                  ? Qt.Checked
                  : Qt.Unchecked
                  onSelectionToggled: printPage.setDaySelected(
                    modelData.local_date,
                    !printPage.daySelected(modelData.local_date)
                  )
                  onDetailsRequested: root.dayDetailDialogObject.openFor(
                    printPage.ticketId,
                    modelData
                  )
                }
              }
            }
          }
        }

        EmptyState {
          visible: printPage.visibleMonthGroups().length === 0
          text: LocaleText.tr("No sales in this period")
        }
      }
    }
  }
}
